import { defineConfig, devices } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

const startScript = path.join(__dirname, 'start-server.sh');

/** Parse a KEY=VALUE env file, skipping blank lines and comments. */
function loadEnv(filePath: string): Record<string, string> {
  const env: Record<string, string> = {};
  for (const line of fs.readFileSync(filePath, 'utf-8').split('\n')) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) continue;
    const idx = trimmed.indexOf('=');
    if (idx === -1) continue;
    env[trimmed.slice(0, idx)] = trimmed.slice(idx + 1);
  }
  return env;
}

const commonEnv = loadEnv(path.join(__dirname, '../common/server.env'));

console.log(`starting playwright with env BUILD_TARGET=${process.env.BUILD_TARGET ?? "debug"}`);

const workspace=process.env.GITHUB_WORKSPACE ?? path.join(__dirname, '../..');

export default defineConfig({
  testDir: './scenarios',
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: process.env.CI ? [['github'], ['html']] : 'html',

  globalSetup: require.resolve('./global-setup'),
  globalTeardown: require.resolve('./global-teardown'),

  use: {
    baseURL: 'http://localhost:8087',
    trace: 'on-first-retry',
  },

  expect: {
    toHaveScreenshot: { maxDiffPixelRatio: 0.02 },
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],

  webServer: {
    command: process.env.BUILD_TARGET
      ? `bash "${startScript}" "${workspace}/target/${process.env.BUILD_TARGET}/oxicloud"`
      : `bash "${startScript}" cargo run`,
    url: 'http://localhost:8087',
    timeout: 600_000,
    reuseExistingServer: false,
    cwd: '../..',
    stdout: 'inherit',
    stderr: 'inherit',
    env: {
      ...commonEnv,
      OXICLOUD_SERVER_PORT: '8087',
      OXICLOUD_STORAGE_PATH: './tests/e2e/storage',
    },
  },
});
