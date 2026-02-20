/**
 * User menu, profile modal and logout logic
 */

function setupUserMenu() {
    const wrapper = document.getElementById('user-menu-wrapper');
    const avatarBtn = document.getElementById('user-avatar-btn');
    const menu = document.getElementById('user-menu');
    const logoutBtn = document.getElementById('user-menu-logout');
    const themeBtn = document.getElementById('user-menu-theme');
    const aboutBtn = document.getElementById('user-menu-about');
    const adminBtn = document.getElementById('user-menu-admin');
    const adminDivider = document.getElementById('user-menu-admin-divider');
    const profileBtn = document.getElementById('user-menu-profile');
    const roleBadge = document.getElementById('user-menu-role-badge');

    if (!wrapper || !avatarBtn || !menu) return;

    avatarBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = wrapper.classList.contains('open');
        wrapper.classList.toggle('open');

        const notifWrapper = document.getElementById('notif-wrapper');
        const notifBtn = document.getElementById('notif-bell-btn');
        if (notifWrapper) notifWrapper.classList.remove('open');
        if (notifBtn) notifBtn.classList.remove('active');

        if (!isOpen) {
            updateUserMenuData();
            const USER_DATA_KEY = 'oxicloud_user';
            const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
            const isAdmin = userData.role === 'admin';
            if (adminBtn) adminBtn.style.display = isAdmin ? 'flex' : 'none';
            if (adminDivider) adminDivider.style.display = isAdmin ? 'block' : 'none';
            if (roleBadge) roleBadge.style.display = isAdmin ? 'block' : 'none';
        }
    });

    document.addEventListener('click', (e) => {
        if (wrapper.classList.contains('open') && !wrapper.contains(e.target)) {
            wrapper.classList.remove('open');
        }
    });

    if (logoutBtn) {
        logoutBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            logout();
        });
    }

    if (themeBtn) {
        const pill = document.getElementById('theme-toggle-pill');
        const isDark = localStorage.getItem('oxicloud_theme') === 'dark';
        if (isDark) {
            if (pill) pill.classList.add('active');
            document.documentElement.setAttribute('data-theme', 'dark');
        }

        themeBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            if (pill) {
                pill.classList.toggle('active');
                const dark = pill.classList.contains('active');
                localStorage.setItem('oxicloud_theme', dark ? 'dark' : 'light');
                document.documentElement.setAttribute('data-theme', dark ? 'dark' : 'light');
                window.ui.showNotification(
                    dark ? '🌙' : '☀️',
                    dark ? 'Dark mode enabled' : 'Light mode enabled'
                );
            }
        });
    }

    if (adminBtn) {
        adminBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            window.location.href = '/admin';
        });
    }

    if (profileBtn) {
        profileBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            window.location.href = '/profile';
        });
    }

    if (aboutBtn) {
        aboutBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            const overlay = document.getElementById('about-modal-overlay');
            if (overlay) overlay.classList.add('show');
        });
    }

    const aboutCloseBtn = document.getElementById('about-close-btn');
    const aboutOverlay = document.getElementById('about-modal-overlay');
    if (aboutCloseBtn) {
        aboutCloseBtn.addEventListener('click', () => {
            aboutOverlay.classList.remove('show');
        });
    }
    if (aboutOverlay) {
        aboutOverlay.addEventListener('click', (e) => {
            if (e.target === aboutOverlay) {
                aboutOverlay.classList.remove('show');
            }
        });
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && aboutOverlay.classList.contains('show')) {
                aboutOverlay.classList.remove('show');
            }
        });
    }

    fetchAppVersion();
}

function updateUserMenuData() {
    const USER_DATA_KEY = 'oxicloud_user';
    const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');

    const nameEl = document.getElementById('user-menu-name');
    const emailEl = document.getElementById('user-menu-email');
    const avatarEl = document.getElementById('user-menu-avatar');
    const storageFill = document.getElementById('user-menu-storage-fill');
    const storageText = document.getElementById('user-menu-storage-text');

    if (userData.username) {
        if (nameEl) nameEl.textContent = userData.username;
        if (emailEl) emailEl.textContent = userData.email || '';
        if (avatarEl) avatarEl.textContent = userData.username.substring(0, 2).toUpperCase();
    }

    const usedBytes = userData.storage_used_bytes || 0;
    const quotaBytes = userData.storage_quota_bytes || (10 * 1024 * 1024 * 1024);
    const percentage = quotaBytes > 0 ? Math.min(Math.round((usedBytes / quotaBytes) * 100), 100) : 0;

    if (storageFill) storageFill.style.width = percentage + '%';
    if (storageText) {
        const used = window.formatFileSize(usedBytes);
        const total = window.formatFileSize(quotaBytes);
        storageText.textContent = `${percentage}% · ${used} / ${total}`;
    }
}

async function fetchAppVersion() {
    try {
        const response = await fetch('/api/version');
        if (response.ok) {
            const data = await response.json();
            const versionEl = document.getElementById('about-version');
            if (versionEl && data.version) {
                versionEl.textContent = `v${data.version}`;
            }
        }
    } catch (err) {
        console.warn('Could not fetch app version:', err);
    }
}

function showUserProfileModal() {
    const USER_DATA_KEY = 'oxicloud_user';
    const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
    const username = userData.username || 'User';
    const email = userData.email || '';
    const role = userData.role || 'user';
    const initials = username.substring(0, 2).toUpperCase();
    const usedBytes = userData.storage_used_bytes || 0;
    const quotaBytes = userData.storage_quota_bytes || (10 * 1024 * 1024 * 1024);
    const percentage = quotaBytes > 0 ? Math.min(Math.round((usedBytes / quotaBytes) * 100), 100) : 0;
    const barColor = percentage > 90 ? '#ef4444' : percentage > 70 ? '#f59e0b' : '#22c55e';

    const t = (key, fallback) => (window.i18n && window.i18n.t) ? window.i18n.t(key) || fallback : fallback;

    const existing = document.getElementById('profile-modal-overlay');
    if (existing) existing.remove();

    const overlay = document.createElement('div');
    overlay.id = 'profile-modal-overlay';
    overlay.className = 'about-modal-overlay';
    overlay.innerHTML = `
        <div class="about-modal" style="max-width:380px">
            <div style="text-align:center;padding:20px 20px 0">
                <div style="width:64px;height:64px;border-radius:50%;background:linear-gradient(135deg,#3b82f6,#6366f1);color:#fff;display:inline-flex;align-items:center;justify-content:center;font-size:24px;font-weight:700;margin-bottom:12px">${initials}</div>
                <h3 style="margin:0;font-size:18px;color:#1a1a2e">${username}</h3>
                <p style="margin:4px 0 0;font-size:13px;color:#64748b">${email}</p>
                <span style="display:inline-block;margin-top:8px;padding:2px 10px;border-radius:10px;font-size:11px;font-weight:600;${
                    role === 'admin'
                        ? 'background:#dbeafe;color:#1d4ed8'
                        : 'background:#f1f5f9;color:#64748b'
                }">${role === 'admin' ? '🛡️ Admin' : '👤 ' + t('user_menu.role_user', 'User')}</span>
            </div>
            <div style="padding:16px 20px">
                <div style="font-size:12px;color:#64748b;text-transform:uppercase;letter-spacing:.05em;margin-bottom:6px">
                    <i class="fas fa-database" style="margin-right:4px"></i>${t('storage.title', 'Storage')}
                </div>
                <div style="background:#f1f5f9;border-radius:6px;height:8px;overflow:hidden;margin-bottom:4px">
                    <div style="height:100%;width:${percentage}%;background:${barColor};border-radius:6px;transition:width .3s"></div>
                </div>
                <div style="font-size:12px;color:#64748b;text-align:right">${percentage}% · ${window.formatFileSize(usedBytes)} / ${quotaBytes > 0 ? window.formatFileSize(quotaBytes) : '∞'}</div>
            </div>
            <div style="padding:0 20px 16px;display:flex;justify-content:center">
                <button id="profile-modal-close" style="padding:8px 24px;border:1px solid #e2e8f0;border-radius:8px;background:#fff;color:#334155;font-size:13px;font-weight:600;cursor:pointer;transition:background .15s">${t('actions.close', 'Close')}</button>
            </div>
        </div>
    `;

    document.body.appendChild(overlay);
    requestAnimationFrame(() => overlay.classList.add('show'));

    overlay.querySelector('#profile-modal-close').addEventListener('click', () => {
        overlay.classList.remove('show');
        setTimeout(() => overlay.remove(), 200);
    });
    overlay.addEventListener('click', (e) => {
        if (e.target === overlay) {
            overlay.classList.remove('show');
            setTimeout(() => overlay.remove(), 200);
        }
    });
}

function logout() {
    const TOKEN_KEY = 'oxicloud_token';
    const REFRESH_TOKEN_KEY = 'oxicloud_refresh_token';
    const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
    const USER_DATA_KEY = 'oxicloud_user';

    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem(REFRESH_TOKEN_KEY);
    localStorage.removeItem(TOKEN_EXPIRY_KEY);
    localStorage.removeItem(USER_DATA_KEY);

    sessionStorage.removeItem('redirect_count');
    window.location.href = '/login';
}

window.setupUserMenu = setupUserMenu;
window.showUserProfileModal = showUserProfileModal;
window.logout = logout;
