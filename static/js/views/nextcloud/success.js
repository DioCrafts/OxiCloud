document.getElementById('close-window-btn').addEventListener('click', function () {
    window.close();
});
// Auto-close after 3 seconds
setTimeout(function () {
    window.close();
}, 3000);
