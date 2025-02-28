// Funcionalidad principal de OxiCloud

document.addEventListener('DOMContentLoaded', function() {
    // Inicializar tooltips de Bootstrap
    var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'))
    var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
        return new bootstrap.Tooltip(tooltipTriggerEl)
    });
    
    // Manejo de subida de archivos
    const uploadForm = document.getElementById('uploadForm');
    if (uploadForm) {
        uploadForm.addEventListener('submit', function(e) {
            const fileInput = document.getElementById('file');
            const progressBar = document.getElementById('uploadProgress');
            const progressBarInner = progressBar.querySelector('.progress-bar');
            
            if (fileInput.files.length > 0) {
                // Mostrar barra de progreso
                progressBar.style.display = 'block';
                progressBarInner.style.width = '0%';
                
                // Simular progreso (en una implementación real, esto se conectaría con el progreso real)
                let progress = 0;
                const interval = setInterval(() => {
                    progress += 5;
                    if (progress <= 90) {
                        progressBarInner.style.width = progress + '%';
                        progressBarInner.setAttribute('aria-valuenow', progress);
                    }
                }, 300);
                
                // Limpiar intervalo después del envío
                setTimeout(() => {
                    clearInterval(interval);
                }, 10000);
            }
        });
    }
    
    // Manejo de mensajes flash
    const flashMessages = document.querySelectorAll('.alert');
    if (flashMessages.length > 0) {
        flashMessages.forEach(message => {
            // Eliminar mensajes automáticamente después de 5 segundos
            setTimeout(() => {
                message.classList.remove('show');
                setTimeout(() => {
                    message.remove();
                }, 500);
            }, 5000);
        });
    }
});

// Función para formatear tamaños de archivo
function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// Función para confirmar eliminación de archivos
function confirmDelete(fileId, fileName) {
    if (confirm(`¿Estás seguro de que deseas eliminar "${fileName}"?`)) {
        const form = document.createElement('form');
        form.method = 'POST';
        form.action = `/files/${fileId}`;
        
        const methodInput = document.createElement('input');
        methodInput.type = 'hidden';
        methodInput.name = '_method';
        methodInput.value = 'DELETE';
        
        form.appendChild(methodInput);
        document.body.appendChild(form);
        form.submit();
    }
}