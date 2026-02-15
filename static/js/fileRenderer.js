/**
 * OxiCloud File Renderer Module
 * Optimized rendering for large file lists using virtual rendering
 */

// Configuration
const ITEMS_PER_PAGE = 100; // Number of items to render at once
const ROW_HEIGHT = 80; // Estimated row height for list view
const CARD_HEIGHT = 180; // Estimated height for grid view card
const CARD_WIDTH = 180; // Estimated width for grid view card

class FileRenderer {
  constructor() {
    this.files = [];
    this.folders = [];
    this.currentView = 'grid';
    this.gridContainer = document.getElementById('files-grid');
    this.listContainer = document.getElementById('files-list-view');
    this.visibleItems = {};
    this.offsetY = 0;
    this.totalHeight = 0;
    this.gridColumns = 0;
    this.i18n = window.i18n || { t: key => key };
    
    // Setup intersection observer for lazy loading
    this.setupIntersectionObserver();
    
    // Handle scroll event for virtual scrolling
    this.handleScroll = this.handleScroll.bind(this);
    this.setupScrollListeners();
  }
  
  /**
   * Set up intersection observer for lazy loading
   */
  setupIntersectionObserver() {
    this.observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          const elem = entry.target;
          if (elem.dataset.lazySrc) {
            elem.src = elem.dataset.lazySrc;
            delete elem.dataset.lazySrc;
            this.observer.unobserve(elem);
          }
        }
      });
    }, {
      rootMargin: '200px', // Load images 200px before they come into view
    });
  }
  
  /**
   * Setup scroll listeners for virtual scrolling
   */
  setupScrollListeners() {
    const container = document.querySelector('.files-list');
    if (container) {
      container.addEventListener('scroll', this.handleScroll);
      // Also update on resize
      window.addEventListener('resize', this.updateVisibleItems.bind(this));
    }
  }
  
  /**
   * Handle scroll events for virtual scrolling
   */
  handleScroll() {
    const container = document.querySelector('.files-list');
    if (!container) return;
    
    this.offsetY = container.scrollTop;
    requestAnimationFrame(() => this.updateVisibleItems());
  }
  
  /**
   * Calculate how many items are visible and render only those
   */
  updateVisibleItems() {
    if (!this.files || !this.folders) return;
    
    const container = document.querySelector('.files-list');
    if (!container) return;
    
    const viewportHeight = container.clientHeight;
    const viewportWidth = container.clientWidth;
    
    // Calculate grid columns based on container width and card width
    this.gridColumns = Math.floor(viewportWidth / CARD_WIDTH);
    if (this.gridColumns < 1) this.gridColumns = 1;
    
    if (this.currentView === 'grid') {
      this.updateGridView(viewportHeight);
    } else {
      this.updateListView(viewportHeight);
    }
  }
  
  /**
   * Update grid view with virtual scrolling
   */
  updateGridView(viewportHeight) {
    const allItems = [...this.folders, ...this.files];
    const rows = Math.ceil(allItems.length / this.gridColumns);
    this.totalHeight = rows * CARD_HEIGHT;
    
    // Calculate visible range
    const startRow = Math.floor(this.offsetY / CARD_HEIGHT);
    const visibleRows = Math.ceil(viewportHeight / CARD_HEIGHT) + 1; // +1 for partial rows
    
    const startIdx = startRow * this.gridColumns;
    const endIdx = Math.min(allItems.length, (startRow + visibleRows) * this.gridColumns);
    
    // Generate a map of visible items
    const newVisibleItems = {};
    for (let i = startIdx; i < endIdx; i++) {
      newVisibleItems[i] = true;
    }
    
    // Remove items that are no longer visible
    if (this.gridContainer) {
      const children = Array.from(this.gridContainer.children);
      children.forEach(child => {
        const idx = parseInt(child.dataset.index, 10);
        if (!newVisibleItems[idx]) {
          this.gridContainer.removeChild(child);
        } else {
          // Item is still visible, remove from new items list
          delete newVisibleItems[idx];
        }
      });
    }
    
    // Add new visible items
    const fragment = document.createDocumentFragment();
    Object.keys(newVisibleItems).forEach(idx => {
      const i = parseInt(idx, 10);
      if (i < allItems.length) {
        const item = allItems[i];
        const elem = this.renderGridItem(item, i);
        fragment.appendChild(elem);
      }
    });
    
    if (this.gridContainer) {
      this.gridContainer.appendChild(fragment);
    }
    
    this.visibleItems = { ...this.visibleItems, ...newVisibleItems };
  }
  
  /**
   * Update list view with virtual scrolling
   */
  updateListView(viewportHeight) {
    const allItems = [...this.folders, ...this.files];
    this.totalHeight = allItems.length * ROW_HEIGHT;
    
    // Calculate visible range
    const startIdx = Math.floor(this.offsetY / ROW_HEIGHT);
    const visibleCount = Math.ceil(viewportHeight / ROW_HEIGHT) + 1; // +1 for partial rows
    const endIdx = Math.min(allItems.length, startIdx + visibleCount);
    
    // Generate a map of visible items
    const newVisibleItems = {};
    for (let i = startIdx; i < endIdx; i++) {
      newVisibleItems[i] = true;
    }
    
    // Remove items that are no longer visible
    if (this.listContainer) {
      const children = Array.from(this.listContainer.children);
      // Skip the first child as it's the header
      for (let i = 1; i < children.length; i++) {
        const child = children[i];
        const idx = parseInt(child.dataset.index, 10);
        if (!newVisibleItems[idx]) {
          this.listContainer.removeChild(child);
        } else {
          // Item is still visible, remove from new items list
          delete newVisibleItems[idx];
        }
      }
    }
    
    // Add new visible items
    const fragment = document.createDocumentFragment();
    Object.keys(newVisibleItems).forEach(idx => {
      const i = parseInt(idx, 10);
      if (i < allItems.length) {
        const item = allItems[i];
        const elem = this.renderListItem(item, i);
        fragment.appendChild(elem);
      }
    });
    
    if (this.listContainer) {
      this.listContainer.appendChild(fragment);
    }
    
    this.visibleItems = { ...this.visibleItems, ...newVisibleItems };
  }
  
  /**
   * Render a single grid item (file or folder)
   */
  renderGridItem(item, index) {
    const elem = document.createElement('div');
    elem.className = 'file-card';
    elem.dataset.index = index;
    
    const isFolder = 'parent_id' in item; // Folders have parent_id
    
    if (isFolder) {
      elem.dataset.folderId = item.id;
      elem.dataset.folderName = item.name;
      elem.dataset.parentId = item.parent_id || "";
      
      elem.innerHTML = `
        <div class="file-icon folder-icon">
          <i class="fas fa-folder"></i>
        </div>
        <div class="file-name">${window.escapeHtml(item.name)}</div>
      `;
      
      // Make draggable
      if (item.parent_id) {
        elem.setAttribute('draggable', 'true');
        elem.addEventListener('dragstart', (e) => {
          e.dataTransfer.setData('text/plain', item.id);
          e.dataTransfer.setData('application/oxicloud-folder', 'true');
          elem.classList.add('dragging');
        });
        
        elem.addEventListener('dragend', () => {
          elem.classList.remove('dragging');
          document.querySelectorAll('.drop-target').forEach(el => {
            el.classList.remove('drop-target');
          });
        });
      }
      
      // Click event
      elem.addEventListener('click', () => {
        if (typeof window.selectFolder === 'function') {
          window.selectFolder(item.id, item.name);
        }
      });
      
    } else {
      // File card
      elem.dataset.fileId = item.id;
      elem.dataset.fileName = item.name;
      elem.dataset.folderId = item.folder_id || "";
      
      // Determine icon based on MIME type
      let iconClass = 'fas fa-file';
      
      if (item.mime_type) {
        if (item.mime_type.startsWith('image/')) {
          iconClass = 'fas fa-file-image';
        } else if (item.mime_type.startsWith('text/')) {
          iconClass = 'fas fa-file-alt';
        } else if (item.mime_type.startsWith('video/')) {
          iconClass = 'fas fa-file-video';
        } else if (item.mime_type.startsWith('audio/')) {
          iconClass = 'fas fa-file-audio';
        } else if (item.mime_type === 'application/pdf') {
          iconClass = 'fas fa-file-pdf';
        }
      }
      
      elem.innerHTML = `
        <div class="file-icon">
          <i class="${iconClass}"></i>
        </div>
        <div class="file-name">${window.escapeHtml(item.name)}</div>
      `;
      
      // Make draggable
      elem.setAttribute('draggable', 'true');
      elem.addEventListener('dragstart', (e) => {
        e.dataTransfer.setData('text/plain', item.id);
        elem.classList.add('dragging');
      });
      
      elem.addEventListener('dragend', () => {
        elem.classList.remove('dragging');
        document.querySelectorAll('.drop-target').forEach(el => {
          el.classList.remove('drop-target');
        });
      });
      
      // Click event (preview)
      elem.addEventListener('click', (event) => {
        event.preventDefault();
        event.stopPropagation();
        
        console.log('File clicked:', item);
        
        // Open file in our new inline viewer
        if (window.inlineViewer) {
          console.log('Using inline viewer');
          window.inlineViewer.openFile(item);
        } else {
          console.warn('Inline viewer not available, downloading directly');
          // Fallback to authenticated download if viewer is not available
          if (window.fileOps) {
            window.fileOps.downloadFile(item.id, item.name);
          }
        }
      });
    }
    
    // Add to intersection observer (for future thumbnail support)
    this.observer.observe(elem);
    
    return elem;
  }
  
  /**
   * Render a single list item (file or folder)
   */
  renderListItem(item, index) {
    const elem = document.createElement('div');
    elem.className = 'file-item';
    elem.dataset.index = index;
    
    const isFolder = 'parent_id' in item; // Folders have parent_id
    
    if (isFolder) {
      elem.dataset.folderId = item.id;
      elem.dataset.folderName = item.name;
      elem.dataset.parentId = item.parent_id || "";
      
      // Format date
      const modifiedDate = new Date(item.modified_at * 1000);
      const formattedDate = modifiedDate.toLocaleDateString() + ' ' + 
                         modifiedDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
      
      elem.innerHTML = `
        <div class="name-cell">
          <div class="file-icon folder-icon">
            <i class="fas fa-folder"></i>
          </div>
          <span>${window.escapeHtml(item.name)}</span>
        </div>
        <div>${this.i18n.t('files.file_types.folder')}</div>
        <div>--</div>
        <div>${formattedDate}</div>
      `;
      
      // Make draggable
      if (item.parent_id) {
        elem.setAttribute('draggable', 'true');
        elem.addEventListener('dragstart', (e) => {
          e.dataTransfer.setData('text/plain', item.id);
          e.dataTransfer.setData('application/oxicloud-folder', 'true');
          elem.classList.add('dragging');
        });
        
        elem.addEventListener('dragend', () => {
          elem.classList.remove('dragging');
          document.querySelectorAll('.drop-target').forEach(el => {
            el.classList.remove('drop-target');
          });
        });
      }
      
      // Click event
      elem.addEventListener('click', () => {
        if (typeof window.selectFolder === 'function') {
          window.selectFolder(item.id, item.name);
        }
      });
      
    } else {
      // File item
      elem.dataset.fileId = item.id;
      elem.dataset.fileName = item.name;
      elem.dataset.folderId = item.folder_id || "";
      
      // Determine file type label based on MIME type
      let typeLabel = this.i18n.t('files.file_types.document');
      let iconClass = 'fas fa-file';
      
      if (item.mime_type) {
        if (item.mime_type.startsWith('image/')) {
          iconClass = 'fas fa-file-image';
          typeLabel = this.i18n.t('files.file_types.image');
        } else if (item.mime_type.startsWith('text/')) {
          iconClass = 'fas fa-file-alt';
          typeLabel = this.i18n.t('files.file_types.text');
        } else if (item.mime_type.startsWith('video/')) {
          iconClass = 'fas fa-file-video';
          typeLabel = this.i18n.t('files.file_types.video');
        } else if (item.mime_type.startsWith('audio/')) {
          iconClass = 'fas fa-file-audio';
          typeLabel = this.i18n.t('files.file_types.audio');
        } else if (item.mime_type === 'application/pdf') {
          iconClass = 'fas fa-file-pdf';
          typeLabel = this.i18n.t('files.file_types.pdf');
        }
      }
      
      // Format file size
      const fileSize = this.formatFileSize(item.size);
      
      // Format date
      const modifiedDate = new Date(item.modified_at * 1000);
      const formattedDate = modifiedDate.toLocaleDateString() + ' ' + 
                           modifiedDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
      
      elem.innerHTML = `
        <div class="name-cell">
          <div class="file-icon">
            <i class="${iconClass}"></i>
          </div>
          <span>${window.escapeHtml(item.name)}</span>
        </div>
        <div>${typeLabel}</div>
        <div>${fileSize}</div>
        <div>${formattedDate}</div>
      `;
      
      // Make draggable
      elem.setAttribute('draggable', 'true');
      elem.addEventListener('dragstart', (e) => {
        e.dataTransfer.setData('text/plain', item.id);
        elem.classList.add('dragging');
      });
      
      elem.addEventListener('dragend', () => {
        elem.classList.remove('dragging');
        document.querySelectorAll('.drop-target').forEach(el => {
          el.classList.remove('drop-target');
        });
      });
      
      // Click event (preview)
      elem.addEventListener('click', (event) => {
        event.preventDefault();
        event.stopPropagation();
        
        console.log('File clicked:', item);
        
        // Open file in our new inline viewer
        if (window.inlineViewer) {
          console.log('Using inline viewer');
          window.inlineViewer.openFile(item);
        } else {
          console.warn('Inline viewer not available, downloading directly');
          // Fallback to authenticated download if viewer is not available
          if (window.fileOps) {
            window.fileOps.downloadFile(item.id, item.name);
          }
        }
      });
    }
    
    return elem;
  }
  
  /**
   * Format file size in human-readable format
   */
  formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }
  
  /**
   * Set current view mode (grid or list)
   */
  setView(view) {
    this.currentView = view;
    this.updateVisibleItems();
  }
  
  /**
   * Load and render files and folders
   */
  loadData(folders, files) {
    this.folders = folders || [];
    this.files = files || [];
    
    // Reset containers
    if (this.gridContainer) {
      this.gridContainer.innerHTML = '';
    }
    
    if (this.listContainer) {
      // Preserve the header
      const header = this.listContainer.querySelector('.list-header');
      this.listContainer.innerHTML = '';
      if (header) {
        this.listContainer.appendChild(header);
      }
    }
    
    this.visibleItems = {};
    this.updateVisibleItems();
  }
}

// Create the file renderer when the DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  window.fileRenderer = new FileRenderer();
  
  // Expose the selectFolder function for navigation
  window.selectFolder = (id, name) => {
    // Update current path
    window.currentPath = id;
    
    // Update breadcrumb
    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) {
      const home = breadcrumb.querySelector('.breadcrumb-item');
      breadcrumb.innerHTML = '';
      
      if (home) {
        breadcrumb.appendChild(home);
        
        if (name) {
          const separator = document.createElement('span');
          separator.className = 'breadcrumb-separator';
          separator.textContent = '>';
          breadcrumb.appendChild(separator);
          
          const folderItem = document.createElement('span');
          folderItem.className = 'breadcrumb-item';
          folderItem.textContent = name;
          breadcrumb.appendChild(folderItem);
        }
      }
    }
    
    // Load files for this folder
    window.loadFiles();
  };
});