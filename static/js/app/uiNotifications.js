/**
 * OxiCloud - UI notifications adapter
 * Isolates notification rendering policy from ui.js.
 */

const uiNotifications = {
    show(title, message) {
        if (window.notifications && typeof window.notifications.addNotification === 'function') {
            const normalizedTitle = String(title || '').toLowerCase();
            let icon = 'fa-info-circle';
            let iconClass = 'upload';

            if (normalizedTitle.includes('error') || normalizedTitle.includes('failed') || normalizedTitle.includes('fail')) {
                icon = 'fa-exclamation-circle';
                iconClass = 'error';
            } else if (normalizedTitle.includes('favorite') || normalizedTitle.includes('favorit') || normalizedTitle.includes('fav')) {
                icon = 'fa-star';
                iconClass = 'success';
            } else if (normalizedTitle.includes('delete') || normalizedTitle.includes('removed') || normalizedTitle.includes('trash') || normalizedTitle.includes('rename') || normalizedTitle.includes('complete')) {
                icon = 'fa-check-circle';
                iconClass = 'success';
            }

            window.notifications.addNotification({
                icon,
                iconClass,
                title: title || '',
                text: message || ''
            });
            return;
        }

        let notification = document.querySelector('.notification');
        if (!notification) {
            notification = document.createElement('div');
            notification.className = 'notification';
            notification.innerHTML = `
                <div class="notification-title">${title}</div>
                <div class="notification-message">${message}</div>
            `;
            document.body.appendChild(notification);
        } else {
            notification.querySelector('.notification-title').textContent = title;
            notification.querySelector('.notification-message').textContent = message;
        }

        notification.style.display = 'block';

        setTimeout(() => {
            notification.style.display = 'none';
        }, 5000);
    }
};

window.uiNotifications = uiNotifications;
