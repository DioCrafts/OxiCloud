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
          <div class="inline-viewer-toolbar-right">
            <button class="inline-viewer-fullscreen" title="Toggle Fullscreen (F)"><i class="fas fa-expand"></i></button>
            <div class="inline-viewer-controls">
              <button class="inline-viewer-zoom-out" title="Zoom Out"><i class="fas fa-search-minus"></i></button>
              <button class="inline-viewer-zoom-reset" title="Reset Zoom"><i class="fas fa-expand-arrows-alt"></i></button>
              <button class="inline-viewer-zoom-in" title="Zoom In"><i class="fas fa-search-plus"></i></button>
            </div>
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

    modal.querySelector('.inline-viewer-fullscreen').addEventListener('click', () => {
      this.toggleFullscreen();
    });

    // Close on ESC key, toggle fullscreen on F key
    document.addEventListener('keydown', (e) => {
      if (!modal.classList.contains('active')) return;
      if (e.key === 'Escape') {
        if (modal.querySelector('.inline-viewer-content').classList.contains('inline-viewer-fullscreen')) {
          this.toggleFullscreen();
        } else {
          this.closeViewer();
        }
        return;
      }
      if (e.key === 'f' && !e.ctrlKey && !e.metaKey && !e.altKey
          && document.activeElement.tagName !== 'INPUT'
          && document.activeElement.tagName !== 'TEXTAREA') {
        this.toggleFullscreen();
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
  
  async openFile(file) {
    console.log('Opening file:', file);

    // WOPI editor intercept: open Office documents in the WOPI editor
    // But NOT image files - those should be previewed in the inline viewer
    // Detect images by mime type OR extension (uploads via WebDAV may lack correct mime)
    const ext = (file.name || '').split('.').pop().toLowerCase();
    const imageExts = ['jpg','jpeg','png','gif','svg','webp','bmp','ico','heic','heif','avif','tiff'];
    const isImage = (file.mime_type && file.mime_type.startsWith('image/')) || imageExts.includes(ext);
    if (!isImage && window.wopiEditor && await window.wopiEditor.canEdit(file.name)) {
        window.wopiEditor.openInModal(file.id, file.name, 'edit');
        return;
    }

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
    if (isImage) {
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
      // Hide zoom controls for PDFs (PDF.js has its own)
      controls.style.display = 'none';

      // Show loading indicator
      const loader = document.createElement('div');
      loader.className = 'inline-viewer-loader';
      loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
      container.appendChild(loader);

      // Create PDF.js viewer
      this.createPdfJsViewer(file, container, loader);
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
    else if (file.mime_type && file.mime_type.startsWith('audio/')) {
      // Hide zoom controls for audio
      controls.style.display = 'none';

      // Show loading indicator
      const loader = document.createElement('div');
      loader.className = 'inline-viewer-loader';
      loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
      container.appendChild(loader);

      // Create audio player
      this.createMediaViewer(file, 'audio', container, loader);
    }
    else if (file.mime_type && file.mime_type.startsWith('video/')) {
      // Hide zoom controls for video
      controls.style.display = 'none';

      // Show loading indicator
      const loader = document.createElement('div');
      loader.className = 'inline-viewer-loader';
      loader.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
      container.appendChild(loader);

      // Create video player
      this.createMediaViewer(file, 'video', container, loader);
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
      
      const response = await fetch(`/api/files/${file.id}?inline=true`, { credentials: 'same-origin' });
      
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
      xhr.withCredentials = true;
      
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
  
  // Creates a PDF.js viewer using authenticated blob fetch
  async createPdfJsViewer(file, container, loader) {
    try {
      // Fetch the PDF as a blob (authenticated)
      const xhr = new XMLHttpRequest();
      xhr.open('GET', `/api/files/${file.id}?inline=true`, true);
      xhr.responseType = 'blob';
      xhr.withCredentials = true;

      const blob = await new Promise((resolve, reject) => {
        xhr.onload = function () {
          if (this.status >= 200 && this.status < 300) resolve(this.response);
          else reject(new Error(`HTTP ${this.status}`));
        };
        xhr.onerror = () => reject(new Error('Network error'));
        xhr.send();
      });

      const blobUrl = URL.createObjectURL(blob);
      this.currentBlobUrl = blobUrl;

      if (loader && loader.parentNode) loader.parentNode.removeChild(loader);

      // Create iframe pointing to PDF.js viewer with the blob URL
      const iframe = document.createElement('iframe');
      iframe.className = 'inline-viewer-pdf';
      iframe.src = `/vendor/pdfjs/web/viewer.html?file=${encodeURIComponent(blobUrl)}`;
      iframe.setAttribute('allowfullscreen', 'true');
      container.appendChild(iframe);
    } catch (error) {
      console.error('Error creating PDF.js viewer:', error);
      if (loader && loader.parentNode) loader.parentNode.removeChild(loader);
      this.showErrorMessage(container);
    }
  }

  // Creates an audio or video player using blob URL (authenticated fetch)
  async createMediaViewer(file, mediaType, container, loader) {
    try {
      console.log(`Creating ${mediaType} player for:`, file.name);

      // Fetch file (cookie auto-sent)
      const response = await fetch(`/api/files/${file.id}?inline=true`, { credentials: 'same-origin' });

      if (!response.ok) {
        throw new Error(`Error fetching file: ${response.status} ${response.statusText}`);
      }

      const blob = await response.blob();
      const blobUrl = URL.createObjectURL(blob);

      // Remove loader
      if (loader && loader.parentNode) {
        loader.parentNode.removeChild(loader);
      }

      if (mediaType === 'audio') {
        // Wrapper with icon + player
        const wrapper = document.createElement('div');
        wrapper.className = 'inline-viewer-audio-wrapper';

        const icon = document.createElement('div');
        icon.className = 'inline-viewer-audio-icon';
        icon.innerHTML = '<i class="fas fa-music"></i>';
        wrapper.appendChild(icon);

        const nameEl = document.createElement('div');
        nameEl.className = 'inline-viewer-audio-name';
        nameEl.textContent = file.name;
        wrapper.appendChild(nameEl);

        const audio = document.createElement('audio');
        audio.className = 'inline-viewer-audio';
        audio.controls = true;
        audio.preload = 'metadata';
        audio.src = blobUrl;
        wrapper.appendChild(audio);

        // Fallback message for unsupported codecs
        audio.addEventListener('error', () => {
          console.warn('Audio playback error — codec may not be supported');
          wrapper.innerHTML = '';
          const msg = document.createElement('div');
          msg.className = 'inline-viewer-message';
          msg.innerHTML = `
            <div class="inline-viewer-icon"><i class="fas fa-exclamation-circle"></i></div>
            <div class="inline-viewer-text">
              <p>Your browser cannot play this audio format.</p>
              <p>Click "Download" to save the file.</p>
            </div>
          `;
          wrapper.appendChild(msg);
        });

        container.appendChild(wrapper);
      } else {
        const video = document.createElement('video');
        video.className = 'inline-viewer-video';
        video.controls = true;
        video.preload = 'metadata';
        video.src = blobUrl;
        video.setAttribute('playsinline', 'true');

        // Fallback message for unsupported codecs
        video.addEventListener('error', () => {
          console.warn('Video playback error — codec may not be supported');
          if (video.parentNode) {
            video.parentNode.removeChild(video);
          }
          const msg = document.createElement('div');
          msg.className = 'inline-viewer-message';
          msg.innerHTML = `
            <div class="inline-viewer-icon"><i class="fas fa-exclamation-circle"></i></div>
            <div class="inline-viewer-text">
              <p>Your browser cannot play this video format.</p>
              <p>Click "Download" to save the file.</p>
            </div>
          `;
          container.appendChild(msg);
        });

        container.appendChild(video);
      }

      // Store blob URL for cleanup on close
      this.currentBlobUrl = blobUrl;
    } catch (error) {
      console.error(`Error creating ${mediaType} viewer:`, error);

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
  
  toggleFullscreen() {
    const content = document.querySelector('#inline-viewer-modal .inline-viewer-content');
    const btn = document.querySelector('.inline-viewer-fullscreen i');
    content.classList.toggle('inline-viewer-fullscreen');
    const isFs = content.classList.contains('inline-viewer-fullscreen');
    btn.className = isFs ? 'fas fa-compress' : 'fas fa-expand';
  }

  closeViewer() {
    // Get modal
    const modal = document.getElementById('inline-viewer-modal');

    // Reset fullscreen state
    const content = modal.querySelector('.inline-viewer-content');
    content.classList.remove('inline-viewer-fullscreen');
    const fsBtn = modal.querySelector('.inline-viewer-fullscreen i');
    if (fsBtn) fsBtn.className = 'fas fa-expand';

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
    fetch(`/api/files/${file.id}`, { credentials: 'same-origin' })
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