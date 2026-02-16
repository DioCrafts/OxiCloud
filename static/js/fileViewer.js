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
    
    // Add to the document
    if (document.body) {
      console.log('Adding viewer container to body');
      document.body.appendChild(this.viewerContainer);
    } else {
      console.warn('Document body not ready, will add viewer later');
      // Try to add it after document is ready
      setTimeout(() => {
        if (document.body) {
          console.log('Adding viewer container to body (delayed)');
          document.body.appendChild(this.viewerContainer);
        } else {
          console.error('Document body still not available after delay');
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
    } else if (fileData.mime_type && this.isTextViewable(fileData.mime_type)) {
      console.log('FileViewer: Loading text viewer');
      this.loadTextViewer(fileData.id, viewerArea);
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
    // Create loader
    const loader = document.createElement('div');
    loader.className = 'file-viewer-loader';
    loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
    container.appendChild(loader);
    
    // Fetch image with auth header and create blob URL
    this.fetchFileAsBlob(fileId).then(blob => {
      const blobUrl = URL.createObjectURL(blob);
      this.currentBlobUrl = blobUrl;
      
      const img = document.createElement('img');
      img.className = 'file-viewer-image';
      img.src = blobUrl;
      img.alt = this.fileData.name;
      
      img.onload = () => {
        if (loader.parentNode) container.removeChild(loader);
      };
      
      img.onerror = () => {
        if (loader.parentNode) container.removeChild(loader);
        this.showErrorMessage(container);
      };
      
      container.appendChild(img);
    }).catch(error => {
      console.error('Error loading image:', error);
      if (loader.parentNode) container.removeChild(loader);
      this.showErrorMessage(container);
    });
    
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
    // Create loader
    const loader = document.createElement('div');
    loader.className = 'file-viewer-loader';
    loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
    container.appendChild(loader);
    
    // Fetch PDF with auth header and create blob URL
    this.fetchFileAsBlob(fileId).then(blob => {
      const blobUrl = URL.createObjectURL(blob);
      this.currentBlobUrl = blobUrl;
      
      const iframe = document.createElement('iframe');
      iframe.className = 'file-viewer-pdf';
      iframe.src = blobUrl;
      iframe.title = this.fileData.name;
      
      iframe.onload = () => {
        if (loader.parentNode) container.removeChild(loader);
      };
      
      container.appendChild(iframe);
    }).catch(error => {
      console.error('Error loading PDF:', error);
      if (loader.parentNode) container.removeChild(loader);
      this.showErrorMessage(container);
    });
  }
  
  /**
   * Load the text viewer
   */
  async loadTextViewer(fileId, container) {
    const loader = document.createElement('div');
    loader.className = 'file-viewer-loader';
    loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
    container.appendChild(loader);
    
    try {
      const token = localStorage.getItem('oxicloud_token');
      const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
      const response = await fetch(`/api/files/${fileId}?inline=true`, { headers });
      
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      
      const text = await response.text();
      if (loader.parentNode) container.removeChild(loader);
      
      const pre = document.createElement('pre');
      pre.className = 'file-viewer-text-content';
      pre.textContent = text;
      container.appendChild(pre);
    } catch (error) {
      console.error('Error loading text:', error);
      if (loader.parentNode) container.removeChild(loader);
      this.showErrorMessage(container);
    }
  }
  
  /**
   * Check if a MIME type is text-viewable — delegates to the single global
   * definition exposed by app.js (window.isTextViewable).
   */
  isTextViewable(mimeType) {
    return window.isTextViewable ? window.isTextViewable(mimeType) : false;
  }
  
  /**
   * Fetch a file as blob with auth headers
   */
  async fetchFileAsBlob(fileId) {
    const token = localStorage.getItem('oxicloud_token');
    const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
    const response = await fetch(`/api/files/${fileId}?inline=true`, { headers });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.blob();
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
      <p>${window.i18n ? window.i18n.t('viewer.unsupported_file') : 'This file type cannot be previewed.'}</p>
      <button class="btn btn-primary download-btn">
        <i class="fas fa-download"></i>
        ${window.i18n ? window.i18n.t('viewer.download_file') : 'Download file'}
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
    
    // Download with auth headers
    const token = localStorage.getItem('oxicloud_token');
    const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
    
    fetch(`/api/files/${this.fileData.id}`, { headers })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.blob();
      })
      .then(blob => {
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = this.fileData.name;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
      })
      .catch(err => console.error('Download error:', err));
  }
  
  /**
   * Show error message
   */
  showErrorMessage(container) {
    const message = document.createElement('div');
    message.className = 'file-viewer-unsupported';
    message.innerHTML = `
      <i class="fas fa-exclamation-triangle"></i>
      <p>Error loading the file. Try downloading it directly.</p>
    `;
    container.appendChild(message);
  }
  
  /**
   * Close the viewer
   */
  close() {
    this.isOpen = false;
    this.fileData = null;
    this.viewerContainer.classList.remove('active');
    
    // Clean up blob URL if exists
    if (this.currentBlobUrl) {
      URL.revokeObjectURL(this.currentBlobUrl);
      this.currentBlobUrl = null;
    }
    
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