import { getCsrfHeaders } from '../../core/csrf.js';
import { i18n } from '../../core/i18n.js';

const API = '/api';

function headers() {
    return { 'Content-Type': 'application/json', ...getCsrfHeaders() };
}

function formatBytes(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024,
        sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / k ** i).toFixed(1))} ${sizes[i]}`;
}

function timeAgo(dateStr) {
    if (!dateStr) return i18n.t('profile.never');
    const d = new Date(dateStr);
    const now = new Date();
    const secs = Math.floor((now - d) / 1000);
    if (secs < 60) return i18n.t('profile.just_now');
    if (secs < 3600) return i18n.t('profile.minutes_ago', { n: Math.floor(secs / 60) });
    if (secs < 86400) return i18n.t('profile.hours_ago', { n: Math.floor(secs / 3600) });
    if (secs < 2592000) return i18n.t('profile.days_ago', { n: Math.floor(secs / 86400) });
    return d.toLocaleDateString();
}

async function init() {
    try {
        const resp = await fetch(`${API}/auth/me`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (!resp.ok) {
            showError();
            return;
        }
        const user = await resp.json();

        const initials = (user.username || '?').substring(0, 2).toUpperCase();
        document.getElementById('p-avatar').textContent = initials;
        document.getElementById('p-username').textContent = user.username;
        document.getElementById('p-email').textContent = user.email || '';

        const badge = document.getElementById('p-role-badge');
        if (user.role === 'admin') {
            badge.className = 'role-badge role-badge-admin';
            badge.innerHTML = `<i class="fas fa-shield-alt"></i> ${i18n.t('profile.role_admin')}`;
        } else {
            badge.className = 'role-badge role-badge-user';
            badge.innerHTML = `<i class="fas fa-user"></i> ${i18n.t('profile.role_user')}`;
        }

        document.getElementById('p-detail-username').textContent = user.username;
        document.getElementById('p-detail-email').textContent = user.email || '—';
        document.getElementById('p-detail-role').textContent = user.role === 'admin' ? i18n.t('profile.role_admin') : i18n.t('profile.role_user');
        document.getElementById('p-detail-login').textContent = timeAgo(user.last_login_at);

        const used = user.storage_used_bytes || 0;
        const quota = user.storage_quota_bytes || 0;
        const pct = quota > 0 ? Math.min(Math.round((used / quota) * 100), 100) : 0;

        document.getElementById('p-storage-used').textContent = formatBytes(used);
        document.getElementById('p-storage-quota').textContent = quota > 0 ? formatBytes(quota) : '∞';
        document.getElementById('p-storage-pct').textContent = quota > 0 ? `${pct}%` : '—';

        const bar = document.getElementById('p-storage-bar');
        bar.style.width = `${pct}%`;
        bar.className = `storage-fill ${pct > 90 ? 'red' : pct > 70 ? 'orange' : 'green'}`;
        document.getElementById('p-storage-text').textContent = `${formatBytes(used)} / ${quota > 0 ? formatBytes(quota) : i18n.t('profile.unlimited')}`;

        if (user.auth_provider && user.auth_provider !== 'local') {
            document.getElementById('password-section').classList.add('hidden');
        }

        loadAppPasswords();

        try {
            const oidcResp = await fetch(`${API}/auth/oidc/providers`, {
                credentials: 'same-origin'
            });
            if (oidcResp.ok) {
                const oidcInfo = await oidcResp.json();
                if (!oidcInfo.password_login_enabled) {
                    document.getElementById('password-section').classList.add('hidden');
                }
            }
        } catch (_oidcErr) {}

        document.getElementById('loading').classList.add('hidden');
        document.getElementById('main-content').classList.remove('hidden');
    } catch (e) {
        console.error(e);
        showError();
    }
}

function showError() {
    document.getElementById('loading').classList.add('hidden');
    document.getElementById('auth-error').classList.remove('hidden');
}

async function changePassword(e) {
    e.preventDefault();
    const currentPw = document.getElementById('current-password').value;
    const newPw = document.getElementById('new-password').value;
    const confirmPw = document.getElementById('confirm-password').value;
    const statusEl = document.getElementById('pw-status');

    if (newPw !== confirmPw) {
        statusEl.innerHTML = `<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ${escapeHtml(i18n.t('profile.passwords_no_match'))}</div>`;
        return false;
    }

    if (newPw.length < 8) {
        statusEl.innerHTML = `<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ${escapeHtml(i18n.t('profile.password_too_short'))}</div>`;
        return false;
    }

    const btn = document.getElementById('pw-submit');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(i18n.t('profile.updating'))}`;

    try {
        const resp = await fetch(`${API}/auth/change-password`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({
                current_password: currentPw,
                new_password: newPw
            })
        });

        if (resp.ok) {
            statusEl.innerHTML = `<div class="alert alert-success"><i class="fas fa-check-circle"></i> ${escapeHtml(i18n.t('profile.password_updated'))}</div>`;
            document.getElementById('password-form').reset();
        } else {
            const err = await resp.json().catch(() => ({}));
            statusEl.innerHTML =
                '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' +
                escapeHtml(err.message || i18n.t('profile.password_change_failed')) +
                '</div>';
        }
    } catch (err) {
        statusEl.innerHTML =
            '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' +
            escapeHtml(i18n.t('profile.error_network', { message: err.message })) +
            '</div>';
    }

    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-save"></i> ${escapeHtml(i18n.t('profile.update_password'))}`;
    return false;
}

// ── App Passwords ──

const AUTO_LABELS = ['Nextcloud', 'Nextcloud (OIDC)'];

function isAutoPassword(pw) {
    return AUTO_LABELS.includes(pw.label);
}

function renderPwRow(pw) {
    const tr = document.createElement('tr');
    const label = document.createElement('td');
    label.textContent = pw.label;
    const created = document.createElement('td');
    created.textContent = new Date(pw.created_at).toLocaleDateString();
    const lastUsed = document.createElement('td');
    lastUsed.textContent = pw.last_used_at ? timeAgo(pw.last_used_at) : i18n.t('profile.never');
    const status = document.createElement('td');
    const badge = document.createElement('span');
    if (pw.active !== false) {
        badge.className = 'badge badge-active';
        badge.textContent = i18n.t('profile.active');
    } else {
        badge.className = 'badge badge-expired';
        badge.textContent = i18n.t('profile.revoked');
    }
    status.appendChild(badge);
    const actions = document.createElement('td');
    if (pw.active !== false) {
        const btn = document.createElement('button');
        btn.className = 'btn btn-danger-sm';
        btn.innerHTML = '<i class="fas fa-trash"></i>';
        btn.title = i18n.t('profile.revoke_title');
        btn.addEventListener('click', () => {
            revokeAppPassword(pw.id, pw.label);
        });
        actions.appendChild(btn);
    }
    tr.append(label, created, lastUsed, status, actions);
    return tr;
}

async function loadAppPasswords() {
    try {
        const resp = await fetch(`${API}/auth/app-passwords`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (!resp.ok) {
            document.getElementById('app-passwords-section').classList.add('hidden');
            return;
        }
        const data = await resp.json();
        const passwords = data.app_passwords || data;
        const userPws = passwords.filter((pw) => {
            return !isAutoPassword(pw);
        });
        const autoPws = passwords.filter(isAutoPassword);

        // User-created passwords
        const tbody = document.getElementById('app-pw-tbody');
        const table = document.getElementById('app-pw-table');
        const empty = document.getElementById('app-pw-empty');
        tbody.innerHTML = '';
        if (userPws.length === 0) {
            table.classList.add('hidden');
            empty.classList.remove('hidden');
        } else {
            table.classList.remove('hidden');
            empty.classList.add('hidden');
            for (const pw of userPws) tbody.appendChild(renderPwRow(pw));
        }

        // Auto-generated (client session) passwords
        const autoSection = document.getElementById('app-pw-auto-section');
        if (autoPws.length === 0) {
            autoSection.classList.add('hidden');
        } else {
            autoSection.classList.remove('hidden');
            document.getElementById('app-pw-auto-count').textContent = autoPws.length;
            const autoTbody = document.getElementById('app-pw-auto-tbody');
            autoTbody.innerHTML = '';
            for (const pw of autoPws) autoTbody.appendChild(renderPwRow(pw));
        }
    } catch (e) {
        console.error('Failed to load app passwords', e);
    }
}

function toggleAutoPasswords() {
    const body = document.getElementById('app-pw-auto-body');
    const chevron = document.getElementById('app-pw-auto-chevron');
    const isHidden = body.classList.contains('hidden');
    body.classList.toggle('hidden', !isHidden);
    chevron.className = isHidden ? 'fas fa-chevron-down' : 'fas fa-chevron-right';
}

async function createAppPassword() {
    const labelInput = document.getElementById('app-pw-label');
    const label = labelInput.value.trim();
    const statusEl = document.getElementById('app-pw-status');
    const btn = document.getElementById('app-pw-generate');

    if (!label) {
        statusEl.innerHTML = `<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ${escapeHtml(i18n.t('profile.error_label_required'))}</div>`;
        return;
    }

    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(i18n.t('profile.generating'))}`;
    statusEl.innerHTML = '';

    try {
        const resp = await fetch(`${API}/auth/app-passwords`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ label: label })
        });
        if (!resp.ok) {
            const err = await resp.json().catch(() => ({}));
            statusEl.innerHTML =
                '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' +
                escapeHtml(err.message || i18n.t('profile.error_create_pw')) +
                '</div>';
            return;
        }
        const result = await resp.json();
        document.getElementById('app-pw-created-label').textContent = result.label;
        document.getElementById('app-pw-created-password').textContent = result.password;
        document.getElementById('app-pw-created').classList.remove('hidden');
        labelInput.value = '';
        loadAppPasswords();
    } catch (err) {
        statusEl.innerHTML = `<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ${err.message}</div>`;
    } finally {
        btn.disabled = false;
        btn.innerHTML = `<i class="fas fa-plus"></i> ${escapeHtml(i18n.t('profile.generate'))}`;
    }
}

function copyAppPassword() {
    const pw = document.getElementById('app-pw-created-password').textContent;
    navigator.clipboard.writeText(pw).then(() => {
        const btn = document.getElementById('app-pw-copy-btn');
        btn.innerHTML = '<i class="fas fa-check"></i>';
        setTimeout(() => {
            btn.innerHTML = '<i class="fas fa-copy"></i>';
        }, 1500);
    });
}

async function revokeAppPassword(id, label) {
    if (!confirm(i18n.t('profile.confirm_revoke', { label: label }))) return;
    try {
        const resp = await fetch(`${API}/auth/app-passwords/${encodeURIComponent(id)}`, {
            method: 'DELETE',
            headers: headers(),
            credentials: 'same-origin'
        });
        if (resp.ok || resp.status === 204) {
            document.getElementById('app-pw-created').classList.add('hidden');
            loadAppPasswords();
        } else {
            const err = await resp.json().catch(() => ({}));
            alert(err.message || i18n.t('profile.error_revoke'));
        }
    } catch (err) {
        alert(i18n.t('profile.error_network', { message: err.message }));
    }
}

function escapeHtml(str) {
    var div = document.createElement('div');
    div.textContent = str || '';
    return div.innerHTML;
}

init();

/* Wire up event handlers (replaces inline onclick/onsubmit) */
document.getElementById('password-form').addEventListener('submit', changePassword);
document.getElementById('app-pw-generate').addEventListener('click', createAppPassword);
document.getElementById('app-pw-copy-btn').addEventListener('click', copyAppPassword);
document.getElementById('app-pw-auto-toggle').addEventListener('click', toggleAutoPasswords);

/* Re-render when language changes */
window.addEventListener('translationsLoaded', () => {
    init();
});
window.addEventListener('localeChanged', () => {
    init();
});
