// Service Worker registration — runs after page load.
if ('serviceWorker' in navigator) {
    window.addEventListener('load', function () {
        navigator.serviceWorker.register('/sw.js')
            .then(function () { /* registered */ })
            .catch(function (err) { console.log('Service Worker registration failed:', err); });
    });
}
