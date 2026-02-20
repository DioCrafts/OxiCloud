/**
 * Trash view loading and rendering logic
 */

async function loadTrashItems() {
    const elements = window.appElements;

    try {
        if (window.multiSelect) window.multiSelect.clear();
        elements.filesGrid.innerHTML = '';
        const _tt = (window.i18n && window.i18n.t) ? window.i18n.t : k => k.split('.').pop();
        elements.filesListView.innerHTML = `
            <div class="list-header trash-header">
                <div data-i18n="files.name">${_tt('files.name')}</div>
                <div data-i18n="files.type">${_tt('files.type')}</div>
                <div data-i18n="trash.original_location">${_tt('trash.original_location')}</div>
                <div data-i18n="trash.deleted_date">${_tt('trash.deleted_date')}</div>
                <div data-i18n="trash.actions">${_tt('trash.actions')}</div>
            </div>
        `;

        window.ui.updateBreadcrumb('');

        const trashItems = await window.fileOps.getTrashItems();

        if (trashItems.length === 0) {
            const emptyState = document.createElement('div');
            emptyState.className = 'empty-state';
            emptyState.innerHTML = `
                <i class="fas fa-trash" style="font-size: 48px; color: #ddd; margin-bottom: 16px;"></i>
                <p>${window.i18n ? window.i18n.t('trash.empty_state') : 'The trash is empty'}</p>
            `;
            elements.filesGrid.appendChild(emptyState);
            return;
        }

        trashItems.forEach(item => {
            addTrashItemToView(item);
        });

    } catch (error) {
        console.error('Error loading trash items:', error);
        window.ui.showNotification('Error', 'Error loading trash items');
    }
}

function addTrashItemToView(item) {
    const elements = window.appElements;
    const isFile = item.item_type === 'file';

    const formattedDate = window.formatDateTime(item.trashed_at);

    let iconClass;
    let typeLabel;
    let iconSpecialClass = '';
    if (!isFile) {
        iconClass = item.icon_class || 'fas fa-folder';
        typeLabel = window.i18n ? window.i18n.t('files.file_types.folder') : 'Folder';
    } else {
        iconClass = item.icon_class || (window.ui && window.ui.getIconClass
            ? window.ui.getIconClass(item.name)
            : 'fas fa-file');
        iconSpecialClass = (window.ui && window.ui.getIconSpecialClass)
            ? window.ui.getIconSpecialClass(item.name)
            : '';
        const cat = item.category || '';
        typeLabel = cat
            ? (window.i18n ? window.i18n.t(`files.file_types.${cat.toLowerCase()}`) || cat : cat)
            : (window.i18n ? window.i18n.t('files.file_types.document') : 'Document');
    }

    const isFolder = !isFile;
    const iconWrapClass = isFolder
        ? 'file-icon folder-icon'
        : `file-icon ${iconSpecialClass}`.trim();

    const gridElement = document.createElement('div');
    gridElement.className = 'file-card trash-item';
    gridElement.dataset.trashId = item.id;
    gridElement.dataset.originalId = item.original_id;
    gridElement.dataset.itemType = item.item_type;
    gridElement.innerHTML = `
        <div class="${iconWrapClass}">
            <i class="${iconClass}"></i>
        </div>
        <div class="file-name">${escapeHtml(item.name)}</div>
        <div class="file-info">${escapeHtml(typeLabel)} - ${escapeHtml(formattedDate)}</div>
        <div class="trash-actions">
            <button class="btn-restore" title="${window.i18n ? window.i18n.t('trash.restore') : 'Restore'}">
                <i class="fas fa-undo"></i>
            </button>
            <button class="btn-delete" title="${window.i18n ? window.i18n.t('trash.delete_permanently') : 'Delete permanently'}">
                <i class="fas fa-trash"></i>
            </button>
        </div>
    `;

    gridElement.querySelector('.btn-restore').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await window.fileOps.restoreFromTrash(item.id)) {
            window.loadTrashItems();
        }
    });

    gridElement.querySelector('.btn-delete').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await window.fileOps.deletePermanently(item.id)) {
            window.loadTrashItems();
        }
    });

    elements.filesGrid.appendChild(gridElement);

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
            <button class="btn-restore" title="${window.i18n ? window.i18n.t('trash.restore') : 'Restore'}">
                <i class="fas fa-undo"></i>
            </button>
            <button class="btn-delete" title="${window.i18n ? window.i18n.t('trash.delete_permanently') : 'Delete permanently'}">
                <i class="fas fa-trash"></i>
            </button>
        </div>
    `;

    listElement.querySelector('.btn-restore').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await window.fileOps.restoreFromTrash(item.id)) {
            window.loadTrashItems();
        }
    });

    listElement.querySelector('.btn-delete').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await window.fileOps.deletePermanently(item.id)) {
            window.loadTrashItems();
        }
    });

    elements.filesListView.appendChild(listElement);
}

window.loadTrashItems = loadTrashItems;
