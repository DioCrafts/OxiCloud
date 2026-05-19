import * as path from 'path';
import * as fs from 'fs/promises';
import { test, expect, Page } from '@playwright/test';
import { loginAsAdmin } from './helpers';

const FIXTURES = path.join(__dirname, '../../fixtures');

/**
 * Opens the new-folder modal, fills the name, and clicks Confirm.
 * Does NOT wait for the modal to close — the caller decides what to assert next.
 */
async function submitNewFolder(page: Page, name: string) {
  await page.locator('#new-folder-btn').click();
  await expect(page.locator('#input-modal')).toBeVisible();
  await page.locator('#modal-input').fill(name);
  await page.locator('#modal-confirm-btn').click();
}

function mimeFromPath(fp: string): string {
  const map: Record<string, string> = {
    '.png':  'image/png',
    '.jpg':  'image/jpeg',
    '.jpeg': 'image/jpeg',
    '.mp4':  'video/mp4',
    '.pdf':  'application/pdf',
  };
  return map[path.extname(fp).toLowerCase()] ?? 'application/octet-stream';
}

/**
 * Simulates a file drag-and-drop onto the OxiCloud dropzone overlay.
 *
 * How it works:
 *  1. File buffers are read in Node and transferred into the page as File
 *     objects inside a DataTransfer — no real OS drag needed.
 *  2. A document-level 'dragover' is dispatched so the app reveals the
 *     dropzone (same code path as a real drag from the OS).
 *  3. A 'drop' is dispatched on #dropzone.  The handler falls back to
 *     dataTransfer.files when webkitGetAsEntry() returns null (which it
 *     does for programmatic File objects), hitting the same upload path.
 */
async function dragFilesToDropzone(page: Page, filePaths: string[]) {
  const fileData = await Promise.all(
    filePaths.map(async (fp) => ({
      name: path.basename(fp),
      buffer: Array.from(await fs.readFile(fp)),
      type: mimeFromPath(fp),
    }))
  );

  const dataTransfer = await page.evaluateHandle((files) => {
    const dt = new DataTransfer();
    for (const { name, buffer, type } of files) {
      dt.items.add(new File([new Uint8Array(buffer)], name, { type }));
    }
    return dt;
  }, fileData);

  // Reveal the dropzone overlay (same logic as a real OS drag).
  await dataTransfer.evaluate((dt) => {
    document.dispatchEvent(new DragEvent('dragover', {
      bubbles: true, cancelable: true, dataTransfer: dt,
    }));
  });
  await expect(page.locator('#dropzone')).toBeVisible();

  await page.locator('#dropzone').dispatchEvent('drop', { dataTransfer });
}

test.describe('Folder management', () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAdmin(page);
  });

  test('folder creation', async ({ page }) => {
    const name = `Test folder ${Date.now()}`;

    await submitNewFolder(page, name);

    await expect(page.locator('#input-modal')).toBeHidden();
    await expect(page.locator(`.file-item[data-folder-name="${name}"]`)).toBeVisible();

    // Screenshot: mask dynamic text so layout is what's tested, not the name.
    await expect(page).toHaveScreenshot('folder-created.png', {
      animations: 'disabled',
      mask: [
        page.locator('.storage-bar'),
        page.locator('.storage-info'),
        page.locator('.date-cell'),
      ],
    });
  });

  test('folder reject if already exists', async ({ page }) => {
    const name = `Test folder ${Date.now()}`;

    // Prerequisite: create the folder once successfully.
    await submitNewFolder(page, name);
    await expect(page.locator('#input-modal')).toBeHidden();

    // Second creation with the same name must keep the modal open with an error.
    await submitNewFolder(page, name);
    await expect(page.locator('#modal-error')).toBeVisible();
    await expect(page.locator('#modal-error')).toContainText('already exists');

    // Cancel leaves the list unchanged.
    await page.locator('#modal-cancel-btn').click();
    await expect(page.locator('#input-modal')).toBeHidden();
  });

  test('folder creation rejected if bad name', async ({ page }) => {
    await page.locator('#new-folder-btn').click();
    await expect(page.locator('#input-modal')).toBeVisible();
    await page.locator('#modal-input').fill('/');
    await page.locator('#modal-confirm-btn').click();

    await expect(page.locator('#modal-error')).toBeVisible();
    await expect(page.locator('#modal-error')).toContainText('Invalid folder name');

    await page.locator('#modal-cancel-btn').click();
    await expect(page.locator('#input-modal')).toBeHidden();
  });

  test('folder rename', async ({ page }) => {
    const ts = Date.now();
    const original = `Test folder ${ts}`;
    const renamed = `Renamed folder ${ts}`;

    // Prerequisite: create the folder to rename.
    await submitNewFolder(page, original);
    await expect(page.locator('#input-modal')).toBeHidden();
    await expect(page.locator(`.file-item[data-folder-name="${original}"]`)).toBeVisible();

    // Open context menu and trigger rename.
    await page.locator(`.file-item[data-folder-name="${original}"] .file-actions`).click();
    await expect(page.locator('#folder-context-menu')).toBeVisible();
    await page.locator('#rename-folder-option').click();

    await expect(page.locator('#input-modal')).toBeVisible();
    await page.locator('#modal-input').fill(renamed);
    await page.locator('#modal-confirm-btn').click();
    await expect(page.locator('#input-modal')).toBeHidden();

    await expect(page.locator(`.file-item[data-folder-name="${renamed}"]`)).toBeVisible();
    await expect(page.locator(`.file-item[data-folder-name="${original}"]`)).not.toBeVisible();

    await expect(page).toHaveScreenshot('folder-renamed.png', {
      animations: 'disabled',
      mask: [
        page.locator('.storage-bar'),
        page.locator('.storage-info'),
        page.locator('.date-cell'),
      ],
    });
  });

  // ── File upload ────────────────────────────────────────────────────────────
  // These two tests are intentionally sequential: test 1 uploads the files,
  // test 2 reads the state left by test 1.  Both run inside the same describe
  // block so Playwright executes them in order within a single worker.

  test('upload of multiple files', async ({ page }) => {
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('#upload-btn').click();
    await expect(page.locator('#upload-dropdown-menu')).toBeVisible();
    await page.locator('#upload-files-btn').click();

    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles([
      path.join(FIXTURES, 'oxicloud-logo.jpg'),
      path.join(FIXTURES, 'free_video_over_1MB.mp4'),
    ]);

    // Each file appears in the list as soon as its individual upload completes.
    await expect(page.locator('.file-item[data-file-name="oxicloud-logo.jpg"]')).toBeVisible({ timeout: 15_000 });
    await expect(page.locator('.file-item[data-file-name="free_video_over_1MB.mp4"]')).toBeVisible({ timeout: 15_000 });
  });

  test('uploaded files rendered with thumbnails', async ({ page }) => {
    // The beforeEach login re-loads the home folder; files uploaded in the
    // previous test are already in the DB and visible immediately.
    await expect(page.locator('.file-item[data-file-name="oxicloud-logo.jpg"]')).toBeVisible();
    await expect(page.locator('.file-item[data-file-name="free_video_over_1MB.mp4"]')).toBeVisible();

    // Image thumbnail is generated server-side: wait for the <img> to be
    // visible (the error handler hides it while the server is still processing,
    // then un-hides it once the src resolves successfully).
    await expect(
      page.locator('.file-item[data-file-name="oxicloud-logo.jpg"] .file-thumb')
    ).toBeVisible({ timeout: 10_000 });

    // Video thumbnail is generated client-side from a canvas frame extraction.
    // Give the browser 1 s to finish before taking the screenshot.
    await page.waitForTimeout(1_000);

    await expect(page).toHaveScreenshot('files-with-thumbnails.png', {
      animations: 'disabled',
      mask: [
        page.locator('.storage-bar'),
        page.locator('.storage-info'),
        page.locator('.date-cell'),   // upload timestamps differ every run
      ],
    });
  });

  test('drag and drop images onto dropzone', async ({ page }) => {
    await dragFilesToDropzone(page, [
      path.join(FIXTURES, 'blue-image.png'),
      path.join(FIXTURES, 'green-image.png'),
      path.join(FIXTURES, 'red-image.png'),
    ]);

    // Each card appears as its upload completes.
    await expect(page.locator('.file-item[data-file-name="blue-image.png"]')).toBeVisible({ timeout: 15_000 });
    await expect(page.locator('.file-item[data-file-name="green-image.png"]')).toBeVisible({ timeout: 15_000 });
    await expect(page.locator('.file-item[data-file-name="red-image.png"]')).toBeVisible({ timeout: 15_000 });

    // PNGs get server-side thumbnails — wait for at least the first to resolve.
    await expect(
      page.locator('.file-item[data-file-name="blue-image.png"] .file-thumb')
    ).toBeVisible({ timeout: 10_000 });

    await expect(page).toHaveScreenshot('dropped-files.png', {
      animations: 'disabled',
      mask: [
        page.locator('.storage-bar'),
        page.locator('.storage-info'),
        page.locator('.date-cell'),
      ],
    });
  });
});
