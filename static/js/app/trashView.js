/**
 * Trash view loading and rendering logic
 */

import { escapeHtml, formatDateTime } from '../core/formatters.js';
import { i18n } from '../core/i18n.js';
import { fileOps } from '../features/files/fileOperations.js';
import { multiSelect } from '../features/files/multiSelect.js';
import { appElements } from './state.js';
import { ui } from './ui.js';

async function loadTrashItems() {
    const elements = appElements;

    try {
        if (multiSelect) multiSelect.clear();
        ui.resetFilesList(); // ensure also list visible & error hidden
        elements.filesList.innerHTML = `
            <div class="list-header trash-header">
                <div data-i18n="files.name">${i18n.t('files.name')}</div>
                <div data-i18n="files.type">${i18n.t('files.type')}</div>
                <div data-i18n="trash.original_location">${i18n.t('trash.original_location')}</div>
                <div data-i18n="trash.deleted_date">${i18n.t('trash.deleted_date')}</div>
                <div data-i18n="trash.actions">${i18n.t('trash.actions')}</div>
            </div>
        `;

        ui.updateBreadcrumb('');

        const trashItems = await fileOps.getTrashItems();

        if (trashItems.length === 0) {
            ui.showError(`
                <i class="fas fa-trash empty-state-icon"></i>
                <p>${i18n.t('trash.empty_state')}</p>
            `);
            return;
        }

        trashItems.forEach((item) => {
            addTrashItemToView(item);
        });
    } catch (error) {
        console.error('Error loading trash items:', error);
        ui.showNotification('Error', 'Error loading trash items');
    }
}

function addTrashItemToView(item) {
    const elements = appElements;
    const isFile = item.item_type === 'file';

    const formattedDate = formatDateTime(item.trashed_at);

    let iconClass;
    let typeLabel;
    let iconSpecialClass = '';
    if (!isFile) {
        iconClass = item.icon_class || 'fas fa-folder';
        typeLabel = i18n.t('files.file_types.folder');
    } else {
        iconClass = item.icon_class || (ui?.getIconClass ? ui.getIconClass(item.name) : 'fas fa-file');
        iconSpecialClass = ui?.getIconSpecialClass ? ui.getIconSpecialClass(item.name) : '';
        const cat = item.category || '';
        typeLabel = cat ? i18n.t(`files.file_types.${cat.toLowerCase()}`) || cat : i18n.t('files.file_types.document');
    }

    const isFolder = !isFile;
    const iconWrapClass = isFolder ? 'file-icon folder-icon' : `file-icon ${iconSpecialClass}`.trim();

    const listElement = document.createElement('div');
    listElement.className = 'file-item trash-item';
    listElement.dataset.trashId = item.id;
    listElement.dataset.originalId = item.original_id;
    listElement.dataset.itemType = item.item_type;

    listElement.innerHTML = `
        <div class="name-cell">
            <div class="${iconWrapClass}">
                <i class="${iconClass}"></i>
            </div>
            <span>${escapeHtml(item.name)}</span>
        </div>
        <div class="type-cell">${escapeHtml(typeLabel)}</div>
        <div class="path-cell">${escapeHtml(item.original_path || '--')}</div>
        <div class="date-cell">${escapeHtml(formattedDate)}</div>
        <div class="actions-cell">
            <button class="btn-restore" title="${i18n.t('trash.restore')}">
                <i class="fas fa-undo"></i>
            </button>
            <button class="btn-delete" title="${i18n.t('trash.delete_permanently')}">
                <i class="fas fa-trash"></i>
            </button>
        </div>
    `;

    listElement.querySelector('.btn-restore').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await fileOps.restoreFromTrash(item.id)) {
            loadTrashItems();
        }
    });

    listElement.querySelector('.btn-delete').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await fileOps.deletePermanently(item.id)) {
            loadTrashItems();
        }
    });

    elements.filesList.appendChild(listElement);
}

export { loadTrashItems };
