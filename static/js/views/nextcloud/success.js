document.getElementById('close-window-btn').addEventListener('click', () => {
    window.close();
});
// Auto-close after 3 seconds
setTimeout(() => {
    window.close();
}, 3000);
