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
