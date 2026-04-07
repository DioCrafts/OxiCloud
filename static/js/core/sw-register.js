// Service Worker registration — runs after page load.
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker
            .register('/sw.js')
            .then(() => {
                /* registered */
            })
            .catch((err) => {
                console.log('Service Worker registration failed:', err);
            });
    });
}
