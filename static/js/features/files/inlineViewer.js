/**
 * OxiCloud Inline Viewer
 * A simpler approach to viewing files that doesn't rely on complex DOM manipulation
 */

class InlineViewer {
  constructor() {
    this.setupViewer();
    this.currentFile = null;
  }
  
  setupViewer() {
    // Create the viewer modal if it doesn't exist
    if (document.getElementById('inline-viewer-modal')) {
      return;
    }
    
    // Verify document.body exists
    if (!document.body) {
      console.warn('Document body not available yet for inline viewer, will retry later');
      setTimeout(() => this.setupViewer(), 200);
      return;
    }
    
    // Create modal container
    const modal = document.createElement('div');
    modal.id = 'inline-viewer-modal';
    modal.className = 'inline-viewer-modal';
    modal.innerHTML = `
      <div class="inline-viewer-content">
        <div class="inline-viewer-header">
          <div class="inline-viewer-title">File Viewer</div>
          <button class="inline-viewer-close"><i class="fas fa-times"></i></button>
        </div>
        <div class="inline-viewer-container"></div>
        <div class="inline-viewer-toolbar">
          <button class="inline-viewer-download"><i class="fas fa-download"></i> Download</button>
          <div class="inline-viewer-controls">
            <button class="inline-viewer-zoom-out" title="Zoom Out"><i class="fas fa-search-minus"></i></button>
            <button class="inline-viewer-zoom-reset" title="Reset Zoom"><i class="fas fa-expand"></i></button>
            <button class="inline-viewer-zoom-in" title="Zoom In"><i class="fas fa-search-plus"></i></button>
          </div>
        </div>
      </div>
    `;
    
    // Add to document
    document.body.appendChild(modal);
    
    // Add event listeners
    modal.querySelector('.inline-viewer-close').addEventListener('click', () => {
      this.closeViewer();
    });
    
    modal.querySelector('.inline-viewer-download').addEventListener('click', () => {
      if (this.currentFile) {
        this.downloadFile(this.currentFile);
      }
    });
    
    // Add zoom controls for images
    modal.querySelector('.inline-viewer-zoom-in').addEventListener('click', () => {
      this.zoomImage(1.2);
    });
    
    modal.querySelector('.inline-viewer-zoom-out').addEventListener('click', () => {
      this.zoomImage(0.8);
    });
    
    modal.querySelector('.inline-viewer-zoom-reset').addEventListener('click', () => {
      this.resetZoom();
    });
    
    // Close on ESC key
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && modal.classList.contains('active')) {
        this.closeViewer();
      }
    });
    
    // Click outside to close
    modal.addEventListener('click', (e) => {
      if (e.target === modal) {
        this.closeViewer();
      }
    });
    
    console.log('Inline viewer initialized');
  }
  
  openFile(file) {
    console.log('Opening file:', file);
    this.currentFile = file;
    
    // Get container
    const modal = document.getElementById('inline-viewer-modal');
    const container = modal.querySelector('.inline-viewer-container');
    const title = modal.querySelector('.inline-viewer-title');
    
    // Clear container
    container.innerHTML = '';
    
    // Set title
    title.textContent = file.name;
    
    // Set controls visibility
    const controls = modal.querySelector('.inline-viewer-controls');
    
    // Show viewer based on file type
    if (file.mime_type && file.mime_type.startsWith('image/')) {
      // Show zoom controls
      controls.style.display = 'flex';
      
      // Show loading indicator
      const loader = document.createElement('div');
      loader.className = 'inline-viewer-loader';
      loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
      container.appendChild(loader);
      
      // Create image viewer using a blob URL
      this.createBlobUrlViewer(file, 'image', container, loader);
    } 
    else if (file.mime_type && file.mime_type === 'application/pdf') {
      // Hide zoom controls for PDFs
      controls.style.display = 'none';
      
      // Show loading indicator
      const loader = document.createElement('div');
      loader.className = 'inline-viewer-loader';
      loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
      container.appendChild(loader);
      
      // Create PDF viewer using object tag with blob URL
      this.createBlobUrlViewer(file, 'pdf', container, loader);
    } 
    else if (file.mime_type && this.isTextViewable(file.mime_type)) {
      // Hide zoom controls for text files
      controls.style.display = 'none';
      
      // Show loading indicator
      const loader = document.createElement('div');
      loader.className = 'inline-viewer-loader';
      loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
      container.appendChild(loader);
      
      // Create text viewer using authenticated fetch
      this.createTextViewer(file, container, loader);
    }
    else {
      // Hide zoom controls for unsupported files
      controls.style.display = 'none';
      
      // Show unsupported file message
      const message = document.createElement('div');
      message.className = 'inline-viewer-message';
      message.innerHTML = `
        <div class="inline-viewer-icon"><i class="fas fa-file"></i></div>
        <div class="inline-viewer-text">
          <p>This file type cannot be previewed.</p>
          <p>Click "Download" to get the file.</p>
        </div>
      `;
      container.appendChild(message);
    }
    
    // Show modal
    modal.classList.add('active');
  }
  
  // Check if a MIME type is text-viewable — delegates to window.isTextViewable
  isTextViewable(mimeType) {
    return window.isTextViewable ? window.isTextViewable(mimeType) : false;
  }
  
  // Creates a text viewer using authenticated fetch
  async createTextViewer(file, container, loader) {
    try {
      console.log('Creating text viewer for:', file.name);
      
      const token = localStorage.getItem('oxicloud_token');
      const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
      
      const response = await fetch(`/api/files/${file.id}?inline=true`, { headers });
      
      if (!response.ok) {
        throw new Error(`Error fetching file: ${response.status} ${response.statusText}`);
      }
      
      const text = await response.text();
      
      // Remove loader
      if (loader && loader.parentNode) {
        loader.parentNode.removeChild(loader);
      }
      
      // Create text viewer element
      const pre = document.createElement('pre');
      pre.className = 'inline-viewer-text-content';
      pre.textContent = text;
      container.appendChild(pre);
      
      console.log('Text viewer created successfully');
    } catch (error) {
      console.error('Error creating text viewer:', error);
      
      // Remove loader
      if (loader && loader.parentNode) {
        loader.parentNode.removeChild(loader);
      }
      
      this.showErrorMessage(container);
    }
  }
  
  // Creates a viewer using a Blob URL to avoid content-disposition header
  async createBlobUrlViewer(file, type, container, loader) {
    try {
      console.log('Creating blob URL viewer for:', file.name, 'type:', type);
      
      // Use XMLHttpRequest instead of fetch to get better control over the response
      const xhr = new XMLHttpRequest();
      xhr.open('GET', `/api/files/${file.id}?inline=true`, true);
      xhr.responseType = 'blob';
      
      // Add auth header
      const token = localStorage.getItem('oxicloud_token');
      if (token) {
        xhr.setRequestHeader('Authorization', `Bearer ${token}`);
      }
      
      // Create a promise to handle the XHR
      const response = await new Promise((resolve, reject) => {
        xhr.onload = function() {
          if (this.status >= 200 && this.status < 300) {
            resolve(this.response);
          } else {
            reject(new Error(`Error fetching file: ${this.status} ${this.statusText}`));
          }
        };
        
        xhr.onerror = function() {
          reject(new Error('Network error'));
        };
        
        xhr.send();
      });
      
      // Create blob URL from response
      const blob = response;
      const blobUrl = URL.createObjectURL(blob);
      
      console.log('Created blob URL:', blobUrl.substring(0, 30) + '...');
      
      // Remove loader
      if (loader && loader.parentNode) {
        loader.parentNode.removeChild(loader);
      }
      
      if (type === 'image') {
        console.log('Creating image viewer');
        // Create image element
        const img = document.createElement('img');
        img.className = 'inline-viewer-image';
        img.src = blobUrl;
        img.alt = file.name;
        container.appendChild(img);
        
        // Add loading indicator until image loads
        img.style.opacity = 0;
        img.onload = () => {
          console.log('Image loaded successfully');
          img.style.opacity = 1;
        };
        
        img.onerror = () => {
          console.error('Failed to load image');
          container.removeChild(img);
          this.showErrorMessage(container);
        };
      } 
      else if (type === 'pdf') {
        console.log('Creating PDF viewer');
        
        // Create iframe for PDF (more reliable than object tag)
        const iframe = document.createElement('iframe');
        iframe.className = 'inline-viewer-pdf';
        iframe.src = blobUrl;
        iframe.setAttribute('allowfullscreen', 'true');
        container.appendChild(iframe);
        
        // Monitor iframe for loading issues
        setTimeout(() => {
          if (!iframe.contentDocument || 
              iframe.contentDocument.body.innerHTML === '') {
            console.warn('PDF viewer might be having issues, adding fallback');
            
            // Add fallback embed
            const embed = document.createElement('embed');
            embed.className = 'inline-viewer-pdf-fallback';
            embed.type = 'application/pdf';
            embed.src = blobUrl;
            container.appendChild(embed);
          }
        }, 2000);
      }
      
      // Store blob URL for cleaning up later
      this.currentBlobUrl = blobUrl;
    } 
    catch (error) {
      console.error('Error creating blob URL viewer:', error);
      
      // Remove loader
      if (loader && loader.parentNode) {
        loader.parentNode.removeChild(loader);
      }
      
      this.showErrorMessage(container);
    }
  }
  
  // Helper to show error message
  showErrorMessage(container) {
    // Show error message
    const message = document.createElement('div');
    message.className = 'inline-viewer-message';
    message.innerHTML = `
      <div class="inline-viewer-icon"><i class="fas fa-exclamation-triangle"></i></div>
      <div class="inline-viewer-text">
        <p>Error loading the file.</p>
        <p>Try downloading it directly.</p>
      </div>
    `;
    container.appendChild(message);
  }
  
  closeViewer() {
    // Get modal
    const modal = document.getElementById('inline-viewer-modal');
    
    // Hide modal
    modal.classList.remove('active');
    
    // Clean up blob URL if exists
    if (this.currentBlobUrl) {
      URL.revokeObjectURL(this.currentBlobUrl);
      this.currentBlobUrl = null;
    }
    
    // Clear references
    this.currentFile = null;
  }
  
  downloadFile(file) {
    const token = localStorage.getItem('oxicloud_token');
    const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
    
    fetch(`/api/files/${file.id}`, { headers })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.blob();
      })
      .then(blob => {
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = file.name;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
      })
      .catch(err => console.error('Download error:', err));
  }
  
  zoomImage(factor) {
    const container = document.querySelector('.inline-viewer-container');
    const img = container.querySelector('.inline-viewer-image');
    
    if (!img) return;
    
    // Get current scale
    let scale = img.dataset.scale ? parseFloat(img.dataset.scale) : 1.0;
    
    // Apply zoom factor
    scale *= factor;
    
    // Limit scale
    scale = Math.max(0.1, Math.min(5.0, scale));
    
    // Save scale
    img.dataset.scale = scale;
    
    // Apply scale
    img.style.transform = `scale(${scale})`;
  }
  
  resetZoom() {
    const container = document.querySelector('.inline-viewer-container');
    const img = container.querySelector('.inline-viewer-image');
    
    if (!img) return;
    
    // Reset scale
    img.dataset.scale = 1.0;
    img.style.transform = 'scale(1.0)';
  }
}

// Initialize viewer when document is ready
document.addEventListener('DOMContentLoaded', () => {
  // Check if it's already initialized
  if (!window.inlineViewer) {
    console.log('Initializing inline viewer on DOMContentLoaded');
    window.inlineViewer = new InlineViewer();
  }
});

// Fallback initialization for cases where DOMContentLoaded already fired
if (document.readyState === 'complete' || document.readyState === 'interactive') {
  if (!window.inlineViewer) {
    console.log('Fallback initialization for inline viewer');
    setTimeout(() => {
      window.inlineViewer = new InlineViewer();
    }, 100);
  }
}