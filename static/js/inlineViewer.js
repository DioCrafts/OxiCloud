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
    
    // Check if document.body exists before appending
    if (document.body) {
      document.body.appendChild(modal);
    } else {
      console.warn('Document body not ready, waiting to append viewer');
      // Try to add it after a short delay
      setTimeout(() => {
        if (document.body) {
          document.body.appendChild(modal);
          
          // Re-add event listeners since we're appending the modal late
          this.setupEventListeners(modal);
        } else {
          console.error('Document body still not available after delay');
        }
      }, 100);
      return; // Exit the function to avoid adding event listeners to a non-existent modal
    }
    
    // Add event listeners
    this.setupEventListeners(modal);
    
    console.log('Inline viewer initialized');
  }
  
  // Separate method for event listeners so they can be re-added if needed
  setupEventListeners(modal) {
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
    else {
      // Hide zoom controls for unsupported files
      controls.style.display = 'none';
      
      // Show unsupported file message
      const message = document.createElement('div');
      message.className = 'inline-viewer-message';
      message.innerHTML = `
        <div class="inline-viewer-icon"><i class="fas fa-file"></i></div>
        <div class="inline-viewer-text">
          <p>Este tipo de archivo no puede ser previsualizado.</p>
          <p>Haz clic en "Descargar" para obtener el archivo.</p>
        </div>
      `;
      container.appendChild(message);
    }
    
    // Show modal
    modal.classList.add('active');
  }
  
  // Creates a viewer using a Blob URL to avoid content-disposition header
  async createBlobUrlViewer(file, type, container, loader) {
    try {
      console.log('Creating blob URL viewer for:', file.name, 'type:', type);
      
      // Use XMLHttpRequest instead of fetch to get better control over the response
      const xhr = new XMLHttpRequest();
      xhr.open('GET', `/api/files/${file.id}?inline=true`, true);
      xhr.responseType = 'blob';
      
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
        <p>Error al cargar el archivo.</p>
        <p>Intenta descargarlo directamente.</p>
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
    // Create a link and click it
    const link = document.createElement('a');
    link.href = `/api/files/${file.id}`;
    link.download = file.name;
    link.target = '_blank';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
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

// Initialize viewer
window.inlineViewer = new InlineViewer();