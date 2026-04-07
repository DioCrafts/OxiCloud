// Apply saved theme immediately (render-blocking) to prevent FOUC.
// This file MUST be loaded without "defer" or "async".
if (localStorage.getItem('oxicloud_theme') === 'dark') document.documentElement.setAttribute('data-theme', 'dark');
