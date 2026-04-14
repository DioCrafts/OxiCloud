import { getCsrfHeaders } from '../../core/csrf.js';
import { escapeHtml } from '../../core/formatters.js';
import { i18n } from '../../core/i18n.js';

const API = '/api';
let currentAdminId = '';
let usersPage = 0;
const PAGE_SIZE = 50;
let totalUsers = 0;

/* ── i18n helper — falls back to key if i18n not ready ── */
function t(key, params) {
    if (i18n && typeof i18n.t === 'function') return i18n.t(key, params);
    // fallback: strip prefix and humanise
    return key.split('.').pop().replace(/_/g, ' ');
}

/** Escape a string for safe embedding inside a JS string literal within an HTML attribute. */
function _escJs(s) {
    if (typeof s !== 'string') return '';
    return s.replace(/[^\w .-]/g, (c) => {
        return `\\x${c.charCodeAt(0).toString(16).padStart(2, '0')}`;
    });
}

function hideElement(id) {
    const element = document.getElementById(id);
    if (!element) return;
    element.classList.remove('show-block', 'show-flex');
    element.classList.add('hidden');
}

function showElement(id, mode = 'block') {
    const element = document.getElementById(id);
    if (!element) return;
    element.classList.remove('hidden', 'show-block', 'show-flex');
    if (mode === 'flex') {
        element.classList.add('show-flex');
    } else {
        element.classList.add('show-block');
    }
}

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
    if (!dateStr) return t('admin.never');
    const d = new Date(dateStr);
    const now = new Date();
    const secs = Math.floor((now - d) / 1000);
    if (secs < 60) return t('admin.just_now');
    if (secs < 3600) return t('admin.minutes_ago', { n: Math.floor(secs / 60) });
    if (secs < 86400) return t('admin.hours_ago', { n: Math.floor(secs / 3600) });
    if (secs < 2592000) return t('admin.days_ago', { n: Math.floor(secs / 86400) });
    return d.toLocaleDateString();
}

/* ── Custom confirm modal ── */
function showConfirm(message) {
    return new Promise((resolve) => {
        const overlay = document.getElementById('confirm-modal');
        const msgEl = document.getElementById('confirm-message');
        const yesBtn = document.getElementById('confirm-yes');
        const noBtn = document.getElementById('confirm-cancel');
        msgEl.textContent = message;
        overlay.classList.remove('hidden');
        overlay.classList.add('show-flex');

        function cleanup(result) {
            overlay.classList.remove('show-flex');
            overlay.classList.add('hidden');
            yesBtn.removeEventListener('click', onYes);
            noBtn.removeEventListener('click', onNo);
            overlay.removeEventListener('click', onOverlay);
            resolve(result);
        }
        function onYes() {
            cleanup(true);
        }
        function onNo() {
            cleanup(false);
        }
        function onOverlay(e) {
            if (e.target === overlay) cleanup(false);
        }
        yesBtn.addEventListener('click', onYes);
        noBtn.addEventListener('click', onNo);
        overlay.addEventListener('click', onOverlay);
    });
}

/* ── Tab switching with fade animation ── */
let activeTabName = 'dashboard';

function switchTab(name, el) {
    if (name === activeTabName) return;
    var oldTab = document.getElementById(`tab-${activeTabName}`);
    var newTab = document.getElementById(`tab-${name}`);

    document.querySelectorAll('.admin-tab').forEach((b) => {
        b.classList.remove('active');
    });
    if (el) el.classList.add('active');

    // Fade-out old tab
    if (oldTab) {
        oldTab.classList.add('tab-fade-out');
        oldTab.addEventListener('animationend', function handler() {
            oldTab.removeEventListener('animationend', handler);
            oldTab.classList.remove('active', 'tab-fade-out');
            // Fade-in new tab
            if (newTab) {
                newTab.classList.add('active', 'tab-fade-in');
                newTab.addEventListener('animationend', function handler2() {
                    newTab.removeEventListener('animationend', handler2);
                    newTab.classList.remove('tab-fade-in');
                });
            }
        });
    } else if (newTab) {
        newTab.classList.add('active', 'tab-fade-in');
        newTab.addEventListener('animationend', function handler2() {
            newTab.removeEventListener('animationend', handler2);
            newTab.classList.remove('tab-fade-in');
        });
    }

    activeTabName = name;
    if (name === 'users') loadUsers();
    if (name === 'dashboard') loadDashboard();
    if (name === 'storage') loadStorage();
}

async function loadDashboard() {
    try {
        const resp = await fetch(`${API}/admin/dashboard`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (!resp.ok) return;
        const d = await resp.json();
        document.getElementById('ds-total-users').textContent = d.total_users;
        document.getElementById('ds-active-users').textContent = d.active_users;
        document.getElementById('ds-admin-users').textContent = d.admin_users;
        document.getElementById('ds-version').textContent = `v${d.server_version}`;
        document.getElementById('ds-used').textContent = formatBytes(d.total_used_bytes);
        document.getElementById('ds-quota').textContent = formatBytes(d.total_quota_bytes);
        document.getElementById('ds-usage-pct').textContent = `${d.storage_usage_percent.toFixed(1)}%`;
        const bar = document.getElementById('ds-bar');
        bar.style.width = `${Math.min(d.storage_usage_percent, 100)}%`;
        bar.className = `progress-fill ${d.storage_usage_percent > 90 ? 'red' : d.storage_usage_percent > 70 ? 'orange' : 'green'}`;
        document.getElementById('ds-auth').textContent = d.auth_enabled ? t('admin.enabled') : t('admin.disabled');
        document.getElementById('ds-oidc').textContent = d.oidc_configured ? t('admin.active') : t('admin.off');
        document.getElementById('ds-quotas-flag').textContent = d.quotas_enabled ? t('admin.enabled') : t('admin.disabled');

        if (typeof d.registration_enabled !== 'undefined') {
            document.getElementById('ds-registration').checked = d.registration_enabled;
            if (d.registration_enabled) hideElement('registration-warning');
            else showElement('registration-warning', 'flex');
        }

        if (d.users_over_80_percent > 0) {
            showElement('ds-warn-card');
            document.getElementById('ds-over80').textContent = d.users_over_80_percent;
        }
        if (d.users_over_quota > 0) {
            showElement('ds-danger-card');
            document.getElementById('ds-overquota').textContent = d.users_over_quota;
        }
    } catch (e) {
        console.error('Dashboard error', e);
    }
}

async function loadUsers() {
    const tbody = document.getElementById('users-tbody');
    tbody.innerHTML = `<tr><td colspan="7" class="table-loading-cell"><i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.loading_users'))}</td></tr>`;
    try {
        const resp = await fetch(`${API}/admin/users?limit=${PAGE_SIZE}&offset=${usersPage * PAGE_SIZE}`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (!resp.ok) {
            tbody.innerHTML =
                '<tr><td colspan="7" class="table-status-error"><i class="fas fa-exclamation-circle"></i> ' +
                escapeHtml(t('admin.failed_load_users')) +
                '</td></tr>';
            return;
        }
        const data = await resp.json();
        totalUsers = data.total;
        const users = data.users;
        if (users.length === 0) {
            tbody.innerHTML = `<tr><td colspan="7" class="table-status-empty">${escapeHtml(t('admin.no_users_found'))}</td></tr>`;
            return;
        }

        tbody.innerHTML = users
            .map((u) => {
                const quotaPct = u.storage_quota_bytes > 0 ? (u.storage_used_bytes / u.storage_quota_bytes) * 100 : 0;
                const quotaColor = quotaPct > 90 ? 'red' : quotaPct > 70 ? 'orange' : 'green';
                const quotaText =
                    u.storage_quota_bytes > 0
                        ? `${formatBytes(u.storage_used_bytes)} / ${formatBytes(u.storage_quota_bytes)}`
                        : `${formatBytes(u.storage_used_bytes)} / ∞`;
                const isSelf = u.id === currentAdminId;
                const isOidc = u.auth_provider && u.auth_provider !== 'local';
                const authBadge = isOidc
                    ? '<span class="badge badge-oidc" title="Authenticated via ' +
                      escapeHtml(u.auth_provider) +
                      '"><i class="fas fa-key badge-admin-icon-small"></i> ' +
                      escapeHtml(u.auth_provider) +
                      '</span>'
                    : `<span class="badge badge-local">${escapeHtml(t('admin.local'))}</span>`;
                return (
                    '<tr>' +
                    '<td><div class="user-info"><span class="user-name">' +
                    escapeHtml(u.username) +
                    (isSelf ? ` <span class="user-self-badge">${escapeHtml(t('admin.you_badge'))}</span>` : '') +
                    '</span><span class="user-email">' +
                    escapeHtml(u.email) +
                    '</span></div></td>' +
                    '<td><span class="badge badge-' +
                    escapeHtml(u.role) +
                    '">' +
                    (u.role === 'admin' ? '<i class="fas fa-shield-alt badge-admin-icon-small"></i> ' : '') +
                    escapeHtml(u.role) +
                    '</span></td>' +
                    '<td>' +
                    authBadge +
                    '</td>' +
                    '<td><span class="badge badge-' +
                    (u.active ? 'active' : 'inactive') +
                    '">' +
                    (u.active ? escapeHtml(t('admin.active')) : escapeHtml(t('admin.inactive'))) +
                    '</span></td>' +
                    '<td><div class="quota-bar"><div class="progress-bar quota-progress-fixed"><div class="progress-fill ' +
                    quotaColor +
                    '" data-width="' +
                    Math.min(quotaPct, 100) +
                    '"></div></div><span class="quota-text">' +
                    quotaText +
                    '</span></div></td>' +
                    '<td class="user-last-login-cell">' +
                    timeAgo(u.last_login_at) +
                    '</td>' +
                    '<td><div class="actions-row">' +
                    '<button class="btn btn-sm btn-secondary admin-action-btn" data-action="quota" data-uid="' +
                    _escJs(u.id) +
                    '" data-uname="' +
                    _escJs(u.username) +
                    '" data-quota="' +
                    u.storage_quota_bytes +
                    '" title="' +
                    escapeHtml(t('admin.edit_quota_title')) +
                    '"><i class="fas fa-box"></i></button>' +
                    (isOidc
                        ? ''
                        : '<button class="btn btn-sm btn-secondary admin-action-btn" data-action="reset-pw" data-uid="' +
                          _escJs(u.id) +
                          '" data-uname="' +
                          _escJs(u.username) +
                          '" title="' +
                          escapeHtml(t('admin.reset_password_title')) +
                          '"><i class="fas fa-key"></i></button>') +
                    '<button class="btn btn-sm btn-secondary admin-action-btn" data-action="toggle-role" data-uid="' +
                    _escJs(u.id) +
                    '" data-role="' +
                    _escJs(u.role) +
                    '" title="' +
                    escapeHtml(t('admin.toggle_role_title')) +
                    '"' +
                    (isSelf ? ' disabled' : '') +
                    '><i class="fas fa-' +
                    (u.role === 'admin' ? 'user' : 'crown') +
                    '"></i></button>' +
                    '<button class="btn btn-sm ' +
                    (u.active ? 'btn-danger' : 'btn-success') +
                    ' admin-action-btn" data-action="toggle-active" data-uid="' +
                    _escJs(u.id) +
                    '" data-active="' +
                    u.active +
                    '" title="' +
                    (u.active ? escapeHtml(t('admin.deactivate_title')) : escapeHtml(t('admin.activate_title'))) +
                    '"' +
                    (isSelf && u.active ? ' disabled' : '') +
                    '><i class="fas fa-' +
                    (u.active ? 'ban' : 'check') +
                    '"></i></button>' +
                    '<button class="btn btn-sm btn-danger admin-action-btn" data-action="delete" data-uid="' +
                    _escJs(u.id) +
                    '" data-uname="' +
                    _escJs(u.username) +
                    '" title="' +
                    escapeHtml(t('admin.delete_title')) +
                    '"' +
                    (isSelf ? ' disabled' : '') +
                    '><i class="fas fa-trash-alt"></i></button>' +
                    '</div></td></tr>'
                );
            })
            .join('');

        // Set dynamic progress bar widths (CSP-safe via JS property)
        document.querySelectorAll('.progress-fill[data-width]').forEach((el) => {
            el.style.width = `${el.dataset.width}%`;
            el.removeAttribute('data-width');
        });

        // Wire up admin action buttons (replaces inline onclick handlers)
        document.querySelectorAll('.admin-action-btn').forEach((btn) => {
            btn.addEventListener('click', () => {
                const action = btn.dataset.action;
                if (action === 'quota') openQuotaModal(btn.dataset.uid, btn.dataset.uname, Number(btn.dataset.quota));
                else if (action === 'reset-pw') openResetPasswordModal(btn.dataset.uid, btn.dataset.uname);
                else if (action === 'toggle-role') toggleRole(btn.dataset.uid, btn.dataset.role);
                else if (action === 'toggle-active') toggleActive(btn.dataset.uid, btn.dataset.active === 'true');
                else if (action === 'delete') deleteUser(btn.dataset.uid, btn.dataset.uname);
            });
        });

        const from = usersPage * PAGE_SIZE + 1;
        const to = Math.min((usersPage + 1) * PAGE_SIZE, totalUsers);
        document.getElementById('users-info').textContent = t('admin.showing_users', { from: from, to: to, total: totalUsers });
        document.getElementById('prev-btn').disabled = usersPage === 0;
        document.getElementById('next-btn').disabled = (usersPage + 1) * PAGE_SIZE >= totalUsers;
    } catch (e) {
        tbody.innerHTML =
            '<tr><td colspan="7" class="table-status-error"><i class="fas fa-exclamation-circle"></i> ' +
            escapeHtml(t('admin.error_network', { message: e.message })) +
            '</td></tr>';
    }
}

function prevPage() {
    if (usersPage > 0) {
        usersPage--;
        loadUsers();
    }
}
function nextPage() {
    if ((usersPage + 1) * PAGE_SIZE < totalUsers) {
        usersPage++;
        loadUsers();
    }
}

async function toggleRole(userId, currentRole) {
    const newRole = currentRole === 'admin' ? 'user' : 'admin';
    const ok = await showConfirm(t('admin.confirm_role_change', { role: newRole }));
    if (!ok) return;
    try {
        const resp = await fetch(`${API}/admin/users/${userId}/role`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ role: newRole })
        });
        if (resp.ok) loadUsers();
        else {
            const e = await resp.json();
            alert(e.message || t('admin.error_generic'));
        }
    } catch (e) {
        alert(t('admin.error_network', { message: e.message }));
    }
}

async function toggleActive(userId, currentActive) {
    const msg = currentActive ? t('admin.confirm_deactivate') : t('admin.confirm_activate');
    const ok = await showConfirm(msg);
    if (!ok) return;
    try {
        const resp = await fetch(`${API}/admin/users/${userId}/active`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ active: !currentActive })
        });
        if (resp.ok) loadUsers();
        else {
            const e = await resp.json();
            alert(e.message || t('admin.error_generic'));
        }
    } catch (e) {
        alert(t('admin.error_network', { message: e.message }));
    }
}

async function deleteUser(userId, username) {
    const ok = await showConfirm(t('admin.confirm_delete_user', { name: username }));
    if (!ok) return;
    try {
        const resp = await fetch(`${API}/admin/users/${userId}`, {
            method: 'DELETE',
            headers: headers(),
            credentials: 'same-origin'
        });
        if (resp.ok) {
            loadUsers();
            loadDashboard();
        } else {
            const e = await resp.json();
            alert(e.message || t('admin.error_generic'));
        }
    } catch (e) {
        alert(t('admin.error_network', { message: e.message }));
    }
}

let quotaUserId = '';
function openQuotaModal(userId, username, currentQuota) {
    quotaUserId = userId;
    document.getElementById('qm-username').textContent = username;
    const gb = currentQuota / 1073741824;
    document.getElementById('qm-unit').value = '1073741824';
    document.getElementById('qm-value').value = gb > 0 ? Math.round(gb * 10) / 10 : 0;
    showElement('quota-modal', 'flex');
}
function closeQuotaModal() {
    hideElement('quota-modal');
}

async function saveQuota() {
    const val = parseFloat(document.getElementById('qm-value').value) || 0;
    const unit = parseInt(document.getElementById('qm-unit').value, 10);
    const bytes = Math.round(val * unit);
    try {
        const resp = await fetch(`${API}/admin/users/${quotaUserId}/quota`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ quota_bytes: bytes })
        });
        if (resp.ok) {
            closeQuotaModal();
            loadUsers();
            loadDashboard();
        } else {
            const e = await resp.json();
            alert(e.message || t('admin.error_generic'));
        }
    } catch (e) {
        alert(t('admin.error_network', { message: e.message }));
    }
}

function openCreateUserModal() {
    document.getElementById('cu-username').value = '';
    document.getElementById('cu-password').value = '';
    document.getElementById('cu-email').value = '';
    document.getElementById('cu-role').value = 'user';
    document.getElementById('cu-quota-value').value = '1';
    document.getElementById('cu-quota-unit').value = '1073741824';
    document.getElementById('cu-error').className = 'alert';
    document.getElementById('cu-error').textContent = '';
    showElement('create-user-modal', 'flex');
    setTimeout(() => document.getElementById('cu-username').focus(), 100);
}
function closeCreateUserModal() {
    hideElement('create-user-modal');
}

async function submitCreateUser() {
    const username = document.getElementById('cu-username').value.trim();
    const password = document.getElementById('cu-password').value;
    const email = document.getElementById('cu-email').value.trim() || null;
    const role = document.getElementById('cu-role').value;
    const quotaVal = parseFloat(document.getElementById('cu-quota-value').value) || 0;
    const quotaUnit = parseInt(document.getElementById('cu-quota-unit').value, 10);
    const quotaBytes = Math.round(quotaVal * quotaUnit);

    const errorEl = document.getElementById('cu-error');
    if (username.length < 3) {
        errorEl.textContent = t('admin.error_username_short');
        errorEl.className = 'alert alert-error';
        return;
    }
    if (password.length < 8) {
        errorEl.textContent = t('admin.error_password_short');
        errorEl.className = 'alert alert-error';
        return;
    }

    const btn = document.getElementById('cu-submit');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.creating'))}`;
    try {
        const resp = await fetch(`${API}/admin/users`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({
                username,
                password,
                email,
                role,
                quota_bytes: quotaBytes
            })
        });
        if (resp.ok) {
            closeCreateUserModal();
            loadUsers();
            loadDashboard();
        } else {
            const e = await resp.json().catch(() => ({}));
            errorEl.textContent = e.message || t('admin.error_create_user');
            errorEl.className = 'alert alert-error';
        }
    } catch (e) {
        errorEl.textContent = t('admin.error_network', { message: e.message });
        errorEl.className = 'alert alert-error';
    }
    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-user-plus"></i> ${escapeHtml(t('admin.create_user'))}`;
}

let resetPwUserId = '';
function openResetPasswordModal(userId, username) {
    resetPwUserId = userId;
    document.getElementById('rp-username').textContent = username;
    document.getElementById('rp-password').value = '';
    document.getElementById('rp-error').className = 'alert';
    document.getElementById('rp-error').textContent = '';
    showElement('reset-pw-modal', 'flex');
    setTimeout(() => document.getElementById('rp-password').focus(), 100);
}
function closeResetPasswordModal() {
    hideElement('reset-pw-modal');
}

async function submitResetPassword() {
    const password = document.getElementById('rp-password').value;
    const errorEl = document.getElementById('rp-error');
    if (password.length < 8) {
        errorEl.textContent = t('admin.error_password_short');
        errorEl.className = 'alert alert-error';
        return;
    }

    const btn = document.getElementById('rp-submit');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.resetting'))}`;
    try {
        const resp = await fetch(`${API}/admin/users/${resetPwUserId}/password`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ new_password: password })
        });
        if (resp.ok) {
            closeResetPasswordModal();
        } else {
            const e = await resp.json().catch(() => ({}));
            errorEl.textContent = e.message || t('admin.error_generic');
            errorEl.className = 'alert alert-error';
        }
    } catch (e) {
        errorEl.textContent = t('admin.error_network', { message: e.message });
        errorEl.className = 'alert alert-error';
    }
    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-save"></i> ${escapeHtml(t('admin.reset_btn'))}`;
}

async function toggleRegistration(enabled) {
    if (enabled) hideElement('registration-warning');
    else showElement('registration-warning', 'flex');
    try {
        const resp = await fetch(`${API}/admin/settings/registration`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ registration_enabled: enabled })
        });
        if (!resp.ok) {
            document.getElementById('ds-registration').checked = !enabled;
            if (!enabled) showElement('registration-warning', 'flex');
            else hideElement('registration-warning');
            const e = await resp.json().catch(() => ({}));
            alert(e.message || t('admin.error_generic'));
        }
    } catch (e) {
        document.getElementById('ds-registration').checked = !enabled;
        if (!enabled) showElement('registration-warning', 'flex');
        else hideElement('registration-warning');
        alert(t('admin.error_network', { message: e.message }));
    }
}

document.getElementById('oidc-enabled').addEventListener('change', function () {
    if (this.checked) showElement('oidc-form');
    else hideElement('oidc-form');
});
document.getElementById('disable-password').addEventListener('change', function () {
    if (this.checked) showElement('password-warning', 'flex');
    else hideElement('password-warning');
});

function showOidcStatus(msg, type) {
    const el = document.getElementById('oidc-status');
    el.textContent = msg;
    el.className = `alert alert-${type}`;
}

function copyCallback() {
    const text = document.getElementById('callback-url').textContent;
    navigator.clipboard.writeText(text);
}

async function testConnection() {
    const url = document.getElementById('issuer-url').value.trim();
    if (!url) {
        showOidcStatus('Enter an Issuer URL first', 'error');
        return;
    }
    const btn = document.getElementById('discover-btn');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.discovering'))}`;
    const resultDiv = document.getElementById('discovery-result');
    try {
        const resp = await fetch(`${API}/admin/settings/oidc/test`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ issuer_url: url })
        });
        const r = await resp.json();
        if (r.success) {
            resultDiv.innerHTML =
                '<div class="discovery-result ok"><strong><i class="fas fa-check-circle"></i> ' +
                escapeHtml(r.message) +
                '</strong><dl><dt>Issuer</dt><dd>' +
                escapeHtml(r.issuer || '—') +
                '</dd><dt>Auth Endpoint</dt><dd>' +
                escapeHtml(r.authorization_endpoint || '—') +
                '</dd></dl></div>';
            if (!document.getElementById('provider-name').value && r.provider_name_suggestion)
                document.getElementById('provider-name').value = r.provider_name_suggestion;
        } else {
            resultDiv.innerHTML = `<div class="discovery-result fail"><strong><i class="fas fa-times-circle"></i> ${escapeHtml(r.message)}</strong></div>`;
        }
    } catch (e) {
        resultDiv.innerHTML = `<div class="discovery-result fail"><i class="fas fa-times-circle"></i> Error: ${escapeHtml(e.message)}</div>`;
    }
    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-search"></i> ${escapeHtml(t('admin.auto_discover'))}`;
}

async function saveOidcSettings() {
    const btn = document.getElementById('save-btn');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.saving'))}`;
    const body = {
        enabled: document.getElementById('oidc-enabled').checked,
        issuer_url: document.getElementById('issuer-url').value.trim(),
        client_id: document.getElementById('client-id').value.trim(),
        client_secret: document.getElementById('client-secret').value || null,
        scopes: document.getElementById('scopes').value.trim() || null,
        auto_provision: document.getElementById('auto-provision').checked,
        admin_groups: document.getElementById('admin-groups').value.trim() || null,
        disable_password_login: document.getElementById('disable-password').checked,
        provider_name: document.getElementById('provider-name').value.trim() || null
    };
    try {
        const resp = await fetch(`${API}/admin/settings/oidc`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify(body)
        });
        if (resp.ok) {
            const status = body.enabled ? t('admin.active').toLowerCase() : t('admin.disabled').toLowerCase();
            showOidcStatus(t('admin.settings_saved', { status: status }), 'success');
            loadDashboard();
        } else {
            const e = await resp.json().catch(() => ({}));
            showOidcStatus(`Error: ${e.message || resp.statusText}`, 'error');
        }
    } catch (e) {
        showOidcStatus(t('admin.error_network', { message: e.message }), 'error');
    }
    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-save"></i> ${escapeHtml(t('admin.save_btn'))}`;
}

/* ── Storage tab ── */

// biome-ignore format: keep the following indent
const STORAGE_PRESETS = {
    'custom':        { endpoint: '',                                             region: '',            pathStyle: false },
    'aws':           { endpoint: '',                                             region: 'us-east-1',   pathStyle: false },
    'backblaze':     { endpoint: 'https://s3.{region}.backblazeb2.com',          region: 'us-west-004', pathStyle: false },
    'cloudflare-r2': { endpoint: 'https://{accountId}.r2.cloudflarestorage.com', region: 'auto',        pathStyle: true  },
    'minio':         { endpoint: 'http://localhost:9000',                        region: 'us-east-1',   pathStyle: true  },
    'digitalocean':  { endpoint: 'https://{region}.digitaloceanspaces.com',      region: 'nyc3',        pathStyle: false },
    'wasabi':        { endpoint: 'https://s3.{region}.wasabisys.com',            region: 'us-east-1',   pathStyle: false },
};

function toggleS3Form(visible) {
    if (visible) showElement('storage-s3-form');
    else hideElement('storage-s3-form');
}

function onStoragePresetChange() {
    const preset = document.getElementById('storage-preset').value;
    const p = STORAGE_PRESETS[preset];
    if (!p) return;
    if (p.endpoint) document.getElementById('storage-endpoint-url').value = p.endpoint;
    if (p.region) document.getElementById('storage-region').value = p.region;
    document.getElementById('storage-path-style').checked = p.pathStyle;
}

function showStorageStatus(msg, type) {
    const el = document.getElementById('storage-status');
    el.textContent = msg;
    el.className = `alert alert-${type}`;
}

async function loadStorage() {
    try {
        const resp = await fetch(`${API}/admin/settings/storage`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (!resp.ok) return;
        const s = await resp.json();

        // Backend selector
        document.querySelectorAll('input[name="storage-backend"]').forEach((r) => {
            r.checked = r.value === s.backend;
        });
        toggleS3Form(s.backend === 's3');

        // S3 fields
        document.getElementById('storage-endpoint-url').value = s.s3_endpoint_url || '';
        document.getElementById('storage-bucket').value = s.s3_bucket || '';
        document.getElementById('storage-region').value = s.s3_region || '';
        document.getElementById('storage-access-key').value = '';
        document.getElementById('storage-secret-key').value = '';
        document.getElementById('storage-path-style').checked = s.s3_force_path_style;

        // Secret hints
        if (s.s3_access_key_set) {
            document.getElementById('storage-access-key').placeholder = t('admin.storage_key_placeholder') || 'Leave empty to keep current value';
        }
        if (s.s3_secret_key_set) {
            showElement('storage-secret-hint');
        } else {
            hideElement('storage-secret-hint');
        }

        // ENV badges
        (s.env_overrides || []).forEach((field) => {
            const badge = document.getElementById(`badge-${field}`);
            if (badge) badge.innerHTML = '<span class="badge badge-env">ENV</span>';
        });

        // Status section
        document.getElementById('storage-current-backend').textContent = s.current_backend || '—';
        document.getElementById('storage-total-blobs').textContent = s.total_blobs != null ? s.total_blobs.toLocaleString() : '—';
        document.getElementById('storage-total-size').textContent = s.total_bytes_stored != null ? formatBytes(s.total_bytes_stored) : '—';
        document.getElementById('storage-dedup-ratio').textContent = s.dedup_ratio != null ? `${s.dedup_ratio.toFixed(2)}x` : '—';
    } catch (e) {
        showStorageStatus(t('admin.error_network', { message: e.message }), 'error');
    }

    // Also load migration status
    loadMigrationStatus();
}

async function saveStorageSettings() {
    const btn = document.getElementById('btn-save-storage');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.saving'))}`;

    const backend = document.querySelector('input[name="storage-backend"]:checked').value;
    const body = {
        backend,
        s3_endpoint_url: document.getElementById('storage-endpoint-url').value.trim() || null,
        s3_bucket: document.getElementById('storage-bucket').value.trim() || null,
        s3_region: document.getElementById('storage-region').value.trim() || null,
        s3_access_key: document.getElementById('storage-access-key').value || null,
        s3_secret_key: document.getElementById('storage-secret-key').value || null,
        s3_force_path_style: document.getElementById('storage-path-style').checked
    };

    try {
        const resp = await fetch(`${API}/admin/settings/storage`, {
            method: 'PUT',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify(body)
        });
        if (resp.ok) {
            showStorageStatus(t('admin.storage_saved') || 'Storage settings saved successfully', 'success');
            loadStorage();
        } else {
            const e = await resp.json().catch(() => ({}));
            showStorageStatus(`Error: ${e.message || resp.statusText}`, 'error');
        }
    } catch (e) {
        showStorageStatus(t('admin.error_network', { message: e.message }), 'error');
    }
    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-save"></i> ${escapeHtml(t('admin.storage_save') || 'Save')}`;
}

async function testStorageConnection() {
    const btn = document.getElementById('btn-test-storage');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.testing') || 'Testing...')}`;

    const backend = document.querySelector('input[name="storage-backend"]:checked').value;
    const body = {
        backend,
        s3_endpoint_url: document.getElementById('storage-endpoint-url').value.trim() || null,
        s3_bucket: document.getElementById('storage-bucket').value.trim() || null,
        s3_region: document.getElementById('storage-region').value.trim() || null,
        s3_access_key: document.getElementById('storage-access-key').value || null,
        s3_secret_key: document.getElementById('storage-secret-key').value || null,
        s3_force_path_style: document.getElementById('storage-path-style').checked
    };

    try {
        const resp = await fetch(`${API}/admin/settings/storage/test`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify(body)
        });
        const r = await resp.json();
        if (r.connected) {
            let msg = `${t('admin.storage_test_success') || 'Connection successful'} (${escapeHtml(r.backend_type)})`;
            if (r.available_bytes != null) msg += ` — ${formatBytes(r.available_bytes)} available`;
            showStorageStatus(msg, 'success');
        } else {
            showStorageStatus(`${t('admin.storage_test_failure') || 'Connection failed'}: ${escapeHtml(r.message)}`, 'error');
        }
    } catch (e) {
        showStorageStatus(t('admin.error_network', { message: e.message }), 'error');
    }
    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-vial"></i> ${escapeHtml(t('admin.storage_test_connection') || 'Test Connection')}`;
}

/* ── Migration ── */

let migrationPollTimer = null;

function showMigrationMsg(msg, type) {
    const el = document.getElementById('migration-status-msg');
    el.textContent = msg;
    el.className = `alert alert-${type}`;
    el.style.display = '';
}

function updateMigrationUI(m) {
    // Status badge
    const badge = document.getElementById('migration-status-badge');
    badge.textContent = (m.status || 'idle').charAt(0).toUpperCase() + (m.status || 'idle').slice(1);
    badge.className = `badge badge-migration badge-migration--${m.status || 'idle'}`;

    const isActive = m.status === 'running' || m.status === 'paused';
    const isCompleted = m.status === 'completed';

    // Progress section
    const progressSection = document.getElementById('migration-progress-section');
    progressSection.style.display = isActive || isCompleted ? '' : 'none';

    if (m.total_blobs > 0) {
        const pct = Math.round((m.migrated_blobs / m.total_blobs) * 100);
        document.getElementById('migration-progress-fill').style.width = `${pct}%`;
        document.getElementById('migration-progress-label').textContent =
            `${m.migrated_blobs.toLocaleString()} / ${m.total_blobs.toLocaleString()} blobs (${pct}%)`;
        document.getElementById('migration-bytes-label').textContent = `${formatBytes(m.migrated_bytes)} transferred`;

        if (m.throughput_bytes_per_sec && m.status === 'running') {
            document.getElementById('migration-throughput').textContent = `${formatBytes(Math.round(m.throughput_bytes_per_sec))}/s`;
            const remaining = m.total_blobs - m.migrated_blobs;
            if (remaining > 0 && m.throughput_bytes_per_sec > 0) {
                const avgBlobSize = m.migrated_bytes / Math.max(m.migrated_blobs, 1);
                const etaSecs = Math.round((remaining * avgBlobSize) / m.throughput_bytes_per_sec);
                const etaMin = Math.ceil(etaSecs / 60);
                document.getElementById('migration-eta').textContent = `~${etaMin} min remaining`;
            }
        } else {
            document.getElementById('migration-throughput').textContent = '';
            document.getElementById('migration-eta').textContent = '';
        }
    }

    // Failed blobs section
    const failedSection = document.getElementById('migration-failed-section');
    if (m.failed_blobs && m.failed_blobs.length > 0) {
        failedSection.style.display = '';
        document.getElementById('migration-failed-count').textContent = m.failed_blobs.length;
        document.getElementById('migration-failed-list').textContent = m.failed_blobs.join('\n');
    } else {
        failedSection.style.display = 'none';
    }

    // Button visibility
    document.getElementById('btn-start-migration').style.display = !isActive && !isCompleted ? '' : 'none';
    document.getElementById('btn-pause-migration').style.display = m.status === 'running' ? '' : 'none';
    document.getElementById('btn-resume-migration').style.display = m.status === 'paused' ? '' : 'none';
    document.getElementById('btn-verify-migration').style.display = isCompleted ? '' : 'none';
    document.getElementById('btn-complete-migration').style.display = isCompleted ? '' : 'none';
}

async function loadMigrationStatus() {
    try {
        const resp = await fetch(`${API}/admin/storage/migration`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (!resp.ok) return;
        const m = await resp.json();
        updateMigrationUI(m);

        // Auto-poll while running
        if (m.status === 'running') {
            if (!migrationPollTimer) {
                migrationPollTimer = setInterval(loadMigrationStatus, 2000);
            }
        } else if (migrationPollTimer) {
            clearInterval(migrationPollTimer);
            migrationPollTimer = null;
        }
    } catch (_e) {
        /* ignore */
    }
}

async function startMigration() {
    const btn = document.getElementById('btn-start-migration');
    btn.disabled = true;
    try {
        const resp = await fetch(`${API}/admin/storage/migration/start`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ concurrency: 4 })
        });
        if (resp.ok) {
            showMigrationMsg(t('admin.migration_started') || 'Migration started', 'success');
            loadMigrationStatus();
        } else {
            const e = await resp.json().catch(() => ({}));
            showMigrationMsg(`Error: ${e.message || resp.statusText}`, 'error');
        }
    } catch (e) {
        showMigrationMsg(t('admin.error_network', { message: e.message }), 'error');
    }
    btn.disabled = false;
}

async function pauseMigration() {
    try {
        const resp = await fetch(`${API}/admin/storage/migration/pause`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin'
        });
        if (resp.ok) {
            showMigrationMsg(t('admin.migration_paused_msg') || 'Migration paused', 'success');
            loadMigrationStatus();
        }
    } catch (_e) {
        /* ignore */
    }
}

async function resumeMigration() {
    try {
        const resp = await fetch(`${API}/admin/storage/migration/resume`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin'
        });
        if (resp.ok) {
            showMigrationMsg(t('admin.migration_resumed_msg') || 'Migration resumed', 'success');
            loadMigrationStatus();
        }
    } catch (_e) {
        /* ignore */
    }
}

async function verifyMigration() {
    const btn = document.getElementById('btn-verify-migration');
    btn.disabled = true;
    btn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${escapeHtml(t('admin.migration_verifying') || 'Verifying...')}`;
    const resultDiv = document.getElementById('migration-verify-result');
    try {
        const resp = await fetch(`${API}/admin/storage/migration/verify`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin',
            body: JSON.stringify({ sample_size: 100 })
        });
        const r = await resp.json();
        resultDiv.style.display = '';
        if (r.passed) {
            resultDiv.innerHTML = `<div class="discovery-result ok"><strong><i class="fas fa-check-circle"></i> ${escapeHtml(t('admin.migration_verify_passed') || 'Verification passed')}</strong><p>${r.sample_checked} blobs checked, ${r.pg_blob_count} total in database</p></div>`;
        } else {
            const issues = [];
            if (r.missing_in_target.length) issues.push(`${r.missing_in_target.length} missing`);
            if (r.size_mismatches.length) issues.push(`${r.size_mismatches.length} size mismatches`);
            resultDiv.innerHTML = `<div class="discovery-result fail"><strong><i class="fas fa-times-circle"></i> ${escapeHtml(t('admin.migration_verify_failed') || 'Verification failed')}</strong><p>${issues.join(', ')}</p></div>`;
        }
    } catch (e) {
        resultDiv.style.display = '';
        resultDiv.innerHTML = `<div class="discovery-result fail"><i class="fas fa-times-circle"></i> Error: ${escapeHtml(e.message)}</div>`;
    }
    btn.disabled = false;
    btn.innerHTML = `<i class="fas fa-check-double"></i> ${escapeHtml(t('admin.migration_verify') || 'Verify Integrity')}`;
}

async function completeMigration() {
    try {
        const resp = await fetch(`${API}/admin/storage/migration/complete`, {
            method: 'POST',
            headers: headers(),
            credentials: 'same-origin'
        });
        if (resp.ok) {
            showMigrationMsg(t('admin.migration_completed_msg') || 'Migration finalized. Restart the server to use the new backend.', 'success');
            loadMigrationStatus();
        } else {
            const e = await resp.json().catch(() => ({}));
            showMigrationMsg(`Error: ${e.message || resp.statusText}`, 'error');
        }
    } catch (e) {
        showMigrationMsg(t('admin.error_network', { message: e.message }), 'error');
    }
}

async function init() {
    try {
        const me = await fetch(`${API}/auth/me`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (!me.ok) {
            showAccessDenied();
            return;
        }
        const user = await me.json();
        if (user.role !== 'admin') {
            showAccessDenied();
            return;
        }
        currentAdminId = user.id;

        const oidcResp = await fetch(`${API}/admin/settings/oidc`, {
            headers: headers(),
            credentials: 'same-origin'
        });
        if (oidcResp.ok) {
            const s = await oidcResp.json();
            document.getElementById('oidc-enabled').checked = s.enabled;
            if (s.enabled) showElement('oidc-form');
            else hideElement('oidc-form');
            document.getElementById('provider-name').value = s.provider_name || '';
            document.getElementById('issuer-url').value = s.issuer_url || '';
            document.getElementById('client-id').value = s.client_id || '';
            document.getElementById('scopes').value = s.scopes || 'openid profile email';
            document.getElementById('auto-provision').checked = s.auto_provision;
            document.getElementById('admin-groups').value = s.admin_groups || '';
            document.getElementById('disable-password').checked = s.disable_password_login;
            if (s.disable_password_login) showElement('password-warning', 'flex');
            else hideElement('password-warning');
            document.getElementById('callback-url').textContent = s.callback_url;
            if (s.client_secret_set) showElement('secret-hint');
            (s.env_overrides || []).forEach((field) => {
                const badge = document.getElementById(`badge-${field}`);
                if (badge) badge.innerHTML = '<span class="badge badge-env">ENV</span>';
            });
        }

        await loadDashboard();
        hideElement('loading');
        showElement('main-content');
    } catch (e) {
        console.error(e);
        showAccessDenied();
    }
}

function showAccessDenied() {
    hideElement('loading');
    showElement('access-denied');
}

/* ── Apply i18n when translations load / change ── */
document.addEventListener('translationsLoaded', () => {
    if (i18n?.translatePage) i18n.translatePage();
    // Re-render dynamic content that uses t()
    loadDashboard();
    if (activeTabName === 'users') loadUsers();
});
document.addEventListener('localeChanged', () => {
    if (i18n?.translatePage) i18n.translatePage();
    loadDashboard();
    if (activeTabName === 'users') loadUsers();
});

init();

/* ── Event-listener wiring (replaces inline onclick/onchange) ── */
document.getElementById('tab-btn-dashboard').addEventListener('click', function () {
    switchTab('dashboard', this);
});
document.getElementById('tab-btn-users').addEventListener('click', function () {
    switchTab('users', this);
});
document.getElementById('tab-btn-oidc').addEventListener('click', function () {
    switchTab('oidc', this);
});
document.getElementById('tab-btn-storage').addEventListener('click', function () {
    switchTab('storage', this);
});

document.getElementById('ds-registration').addEventListener('change', function () {
    toggleRegistration(this.checked);
});

document.getElementById('btn-create-user').addEventListener('click', openCreateUserModal);
document.getElementById('prev-btn').addEventListener('click', prevPage);
document.getElementById('next-btn').addEventListener('click', nextPage);

document.getElementById('discover-btn').addEventListener('click', testConnection);
document.getElementById('btn-copy-callback').addEventListener('click', copyCallback);
document.getElementById('btn-test-oidc').addEventListener('click', testConnection);
document.getElementById('save-btn').addEventListener('click', saveOidcSettings);

document.getElementById('btn-close-quota').addEventListener('click', closeQuotaModal);
document.getElementById('btn-save-quota').addEventListener('click', saveQuota);

document.getElementById('btn-close-create-user').addEventListener('click', closeCreateUserModal);
document.getElementById('cu-submit').addEventListener('click', submitCreateUser);

document.getElementById('btn-close-reset-pw').addEventListener('click', closeResetPasswordModal);
document.getElementById('rp-submit').addEventListener('click', submitResetPassword);

/* ── Storage event listeners ── */
document.querySelectorAll('input[name="storage-backend"]').forEach((r) => {
    r.addEventListener('change', function () {
        toggleS3Form(this.value === 's3');
    });
});
document.getElementById('storage-preset').addEventListener('change', onStoragePresetChange);
document.getElementById('btn-test-storage').addEventListener('click', testStorageConnection);
document.getElementById('btn-save-storage').addEventListener('click', saveStorageSettings);

/* ── Migration event listeners ── */
document.getElementById('btn-start-migration').addEventListener('click', startMigration);
document.getElementById('btn-pause-migration').addEventListener('click', pauseMigration);
document.getElementById('btn-resume-migration').addEventListener('click', resumeMigration);
document.getElementById('btn-verify-migration').addEventListener('click', verifyMigration);
document.getElementById('btn-complete-migration').addEventListener('click', completeMigration);
