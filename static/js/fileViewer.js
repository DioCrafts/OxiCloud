/**
 * OxiCloud File Viewer Module
 * Provides integrated viewers for images, PDFs and other file types
 */

class FileViewer {
  constructor() {
    this.viewerContainer = null;
    this.fileData = null;
    this.isOpen = false;
    
    // Create the viewer container on initialization
    this.createViewerContainer();
  }
  
  /**
   * Create the viewer container DOM element
   */
  createViewerContainer() {
    console.log('Creating file viewer container');
    
    // Check if container already exists
    if (document.getElementById('file-viewer-container')) {
      console.log('Viewer container already exists');
      this.viewerContainer = document.getElementById('file-viewer-container');
      return;
    }
    
    // Create viewer container
    this.viewerContainer = document.createElement('div');
    this.viewerContainer.id = 'file-viewer-container';
    this.viewerContainer.className = 'file-viewer-container';
    
    // Create viewer content
    const viewerContent = document.createElement('div');
    viewerContent.className = 'file-viewer-content';
    
    // Create header
    const header = document.createElement('div');
    header.className = 'file-viewer-header';
    
    const title = document.createElement('div');
    title.className = 'file-viewer-title';
    title.textContent = 'File Viewer';
    header.appendChild(title);
    
    const closeBtn = document.createElement('button');
    closeBtn.className = 'file-viewer-close';
    closeBtn.innerHTML = '<i class="fas fa-times"></i>';
    closeBtn.addEventListener('click', () => {
      console.log('Close button clicked');
      this.close();
    });
    header.appendChild(closeBtn);
    
    // Create viewer area
    const viewerArea = document.createElement('div');
    viewerArea.className = 'file-viewer-area';
    
    // Create toolbar
    const toolbar = document.createElement('div');
    toolbar.className = 'file-viewer-toolbar';
    
    const downloadBtn = document.createElement('button');
    downloadBtn.className = 'file-viewer-download';
    downloadBtn.innerHTML = '<i class="fas fa-download"></i>';
    downloadBtn.addEventListener('click', () => this.downloadFile());
    toolbar.appendChild(downloadBtn);
    
    // Assemble the viewer
    viewerContent.appendChild(header);
    viewerContent.appendChild(viewerArea);
    viewerContent.appendChild(toolbar);
    this.viewerContainer.appendChild(viewerContent);
    
    // Store the created container as a class property so we can add it later if needed
    this._preparedContainer = this.viewerContainer;
    
    // Add to the document if body is available, otherwise set up a listener
    if (document.body) {
      console.log('Adding viewer container to body');
      document.body.appendChild(this.viewerContainer);
    } else {
      console.warn('Document body not ready, will add viewer later');
      
      // Set up a MutationObserver to detect when body becomes available
      const observer = new MutationObserver((mutations, obs) => {
        if (document.body) {
          console.log('Body detected, adding viewer container');
          document.body.appendChild(this._preparedContainer);
          obs.disconnect(); // Stop observing once we've added the container
        }
      });
      
      // Observe changes to the document to detect when body is added
      observer.observe(document.documentElement, { childList: true, subtree: true });
      
      // Also set a fallback timeout
      setTimeout(() => {
        if (document.body && !document.getElementById('file-viewer-container')) {
          console.log('Adding viewer container to body (via timeout)');
          document.body.appendChild(this._preparedContainer);
        }
      }, 500);
    }
    
    // Add event listeners for keyboard navigation
    document.addEventListener('keydown', (e) => {
      if (this.isOpen && e.key === 'Escape') {
        console.log('Escape key pressed, closing viewer');
        this.close();
      }
    });
    
    console.log('Viewer container created');
  }
  
  /**
   * Open the viewer with the specified file
   * @param {Object} fileData - File data with id, name, mime_type
   */
  async open(fileData) {
    console.log('FileViewer: Opening file', fileData);
    this.fileData = fileData;
    this.isOpen = true;
    
    // Reset the viewer area
    const viewerArea = this.viewerContainer.querySelector('.file-viewer-area');
    viewerArea.innerHTML = '';
    
    // Set the title
    const title = this.viewerContainer.querySelector('.file-viewer-title');
    title.textContent = fileData.name;
    
    // Show the container
    this.viewerContainer.classList.add('active');
    
    // Determine content type and load appropriate viewer
    if (fileData.mime_type && fileData.mime_type.startsWith('image/')) {
      console.log('FileViewer: Loading image viewer');
      this.loadImageViewer(fileData.id, viewerArea);
    } else if (fileData.mime_type && fileData.mime_type === 'application/pdf') {
      console.log('FileViewer: Loading PDF viewer');
      this.loadPdfViewer(fileData.id, viewerArea);
    } else {
      console.log('FileViewer: Unsupported file type', fileData.mime_type);
      // For unsupported files, show download prompt
      this.showUnsupportedFileMessage(viewerArea);
    }
  }
  
  /**
   * Load the image viewer
   * @param {string} fileId - ID of the file to view
   * @param {HTMLElement} container - Container element to render into
   */
  loadImageViewer(fileId, container) {
    // Create image element
    const img = document.createElement('img');
    img.className = 'file-viewer-image';
    img.src = `/api/files/${fileId}`;
    img.alt = this.fileData.name;
    
    // Create loader
    const loader = document.createElement('div');
    loader.className = 'file-viewer-loader';
    loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
    container.appendChild(loader);
    
    // When image loads, remove loader
    img.onload = () => {
      container.removeChild(loader);
    };
    
    // Add image to container
    container.appendChild(img);
    
    // Add zoom controls to toolbar
    const toolbar = this.viewerContainer.querySelector('.file-viewer-toolbar');
    
    const zoomInBtn = document.createElement('button');
    zoomInBtn.className = 'file-viewer-zoom-in';
    zoomInBtn.innerHTML = '<i class="fas fa-search-plus"></i>';
    zoomInBtn.addEventListener('click', () => this.zoomImage(1.2));
    toolbar.appendChild(zoomInBtn);
    
    const zoomOutBtn = document.createElement('button');
    zoomOutBtn.className = 'file-viewer-zoom-out';
    zoomOutBtn.innerHTML = '<i class="fas fa-search-minus"></i>';
    zoomOutBtn.addEventListener('click', () => this.zoomImage(0.8));
    toolbar.appendChild(zoomOutBtn);
    
    const resetZoomBtn = document.createElement('button');
    resetZoomBtn.className = 'file-viewer-zoom-reset';
    resetZoomBtn.innerHTML = '<i class="fas fa-expand"></i>';
    resetZoomBtn.addEventListener('click', () => this.resetZoom());
    toolbar.appendChild(resetZoomBtn);
  }
  
  /**
   * Zoom the image
   * @param {number} factor - Zoom factor
   */
  zoomImage(factor) {
    const img = this.viewerContainer.querySelector('.file-viewer-image');
    if (!img) return;
    
    // Get current scale
    let scale = img.style.transform ?
      parseFloat(img.style.transform.replace('scale(', '').replace(')', '')) : 1;
    
    // Apply new scale
    scale *= factor;
    
    // Limit scale range
    scale = Math.max(0.5, Math.min(5, scale));
    
    img.style.transform = `scale(${scale})`;
  }
  
  /**
   * Reset image zoom
   */
  resetZoom() {
    const img = this.viewerContainer.querySelector('.file-viewer-image');
    if (!img) return;
    
    img.style.transform = 'scale(1)';
  }
  
  /**
   * Load the PDF viewer
   * @param {string} fileId - ID of the file to view
   * @param {HTMLElement} container - Container element to render into
   */
  loadPdfViewer(fileId, container) {
    // Create iframe for PDF viewer
    const iframe = document.createElement('iframe');
    iframe.className = 'file-viewer-pdf';
    iframe.src = `/api/files/${fileId}`;
    iframe.title = this.fileData.name;
    
    // Create loader
    const loader = document.createElement('div');
    loader.className = 'file-viewer-loader';
    loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
    container.appendChild(loader);
    
    // When iframe loads, remove loader
    iframe.onload = () => {
      container.removeChild(loader);
    };
    
    // Add iframe to container
    container.appendChild(iframe);
  }
  
  /**
   * Show message for unsupported file types
   * @param {HTMLElement} container - Container element to render into
   */
  showUnsupportedFileMessage(container) {
    const message = document.createElement('div');
    message.className = 'file-viewer-unsupported';
    
    message.innerHTML = `
      <i class="fas fa-file-download"></i>
      <p>${window.i18n ? window.i18n.t('viewer.unsupported_file') : 'Este tipo de archivo no se puede previsualizar.'}</p>
      <button class="btn btn-primary download-btn">
        <i class="fas fa-download"></i>
        ${window.i18n ? window.i18n.t('viewer.download_file') : 'Descargar archivo'}
      </button>
    `;
    
    // Add download button click event
    message.querySelector('.download-btn').addEventListener('click', () => {
      this.downloadFile();
    });
    
    container.appendChild(message);
  }
  
  /**
   * Download the current file
   */
  downloadFile() {
    if (!this.fileData) return;
    
    // Create a link and simulate click
    const link = document.createElement('a');
    link.href = `/api/files/${this.fileData.id}`;
    link.download = this.fileData.name;
    link.target = '_blank';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }
  
  /**
   * Close the viewer
   */
  close() {
    this.isOpen = false;
    this.fileData = null;
    this.viewerContainer.classList.remove('active');
    
    // Reset toolbar (remove zoom controls)
    const toolbar = this.viewerContainer.querySelector('.file-viewer-toolbar');
    const downloadBtn = toolbar.querySelector('.file-viewer-download');
    toolbar.innerHTML = '';
    toolbar.appendChild(downloadBtn);
  }
}

// Create the file viewer immediately and make it accessible globally
window.fileViewer = new FileViewer();

// Ensure the file viewer is available when the DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  console.log('FileViewer initialized:', window.fileViewer ? 'Yes' : 'No');
  if (!window.fileViewer) {
    console.warn('Re-initializing fileViewer as it was not properly set');
    window.fileViewer = new FileViewer();
  }
});