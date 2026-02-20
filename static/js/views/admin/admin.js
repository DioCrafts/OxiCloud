const API = '/api';
const token = localStorage.getItem('oxicloud_token') || localStorage.getItem('token') || localStorage.getItem('access_token');
let currentAdminId = '';
let usersPage = 0;
const PAGE_SIZE = 50;
let totalUsers = 0;

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
  return { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' };
}

function formatBytes(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024, sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

function timeAgo(dateStr) {
  if (!dateStr) return 'Never';
  const d = new Date(dateStr);
  const now = new Date();
  const secs = Math.floor((now - d) / 1000);
  if (secs < 60) return 'Just now';
  if (secs < 3600) return Math.floor(secs/60) + 'm ago';
  if (secs < 86400) return Math.floor(secs/3600) + 'h ago';
  if (secs < 2592000) return Math.floor(secs/86400) + 'd ago';
  return d.toLocaleDateString();
}

function switchTab(name, el) {
  document.querySelectorAll('.admin-tab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));
  document.getElementById('tab-' + name).classList.add('active');
  if (el) el.classList.add('active');
  if (name === 'users') loadUsers();
  if (name === 'dashboard') loadDashboard();
}

async function loadDashboard() {
  try {
    const resp = await fetch(API + '/admin/dashboard', { headers: headers() });
    if (!resp.ok) return;
    const d = await resp.json();
    document.getElementById('ds-total-users').textContent = d.total_users;
    document.getElementById('ds-active-users').textContent = d.active_users;
    document.getElementById('ds-admin-users').textContent = d.admin_users;
    document.getElementById('ds-version').textContent = 'v' + d.server_version;
    document.getElementById('ds-used').textContent = formatBytes(d.total_used_bytes);
    document.getElementById('ds-quota').textContent = formatBytes(d.total_quota_bytes);
    document.getElementById('ds-usage-pct').textContent = d.storage_usage_percent.toFixed(1) + '%';
    const bar = document.getElementById('ds-bar');
    bar.style.width = Math.min(d.storage_usage_percent, 100) + '%';
    bar.className = 'progress-fill ' + (d.storage_usage_percent > 90 ? 'red' : d.storage_usage_percent > 70 ? 'orange' : 'green');
    document.getElementById('ds-auth').textContent = d.auth_enabled ? 'Enabled' : 'Disabled';
    document.getElementById('ds-oidc').textContent = d.oidc_configured ? 'Active' : 'Off';
    document.getElementById('ds-quotas-flag').textContent = d.quotas_enabled ? 'Enabled' : 'Disabled';

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
  } catch (e) { console.error('Dashboard error', e); }
}

async function loadUsers() {
  const tbody = document.getElementById('users-tbody');
  tbody.innerHTML = '<tr><td colspan="6" class="table-loading-cell"><i class="fas fa-spinner fa-spin"></i> Loading…</td></tr>';
  try {
    const resp = await fetch(API + '/admin/users?limit=' + PAGE_SIZE + '&offset=' + (usersPage * PAGE_SIZE), { headers: headers() });
    if (!resp.ok) { tbody.innerHTML = '<tr><td colspan="6" class="table-status-error"><i class="fas fa-exclamation-circle"></i> Failed to load users</td></tr>'; return; }
    const data = await resp.json();
    totalUsers = data.total;
    const users = data.users;
    if (users.length === 0) { tbody.innerHTML = '<tr><td colspan="6" class="table-status-empty">No users found</td></tr>'; return; }

    tbody.innerHTML = users.map(u => {
      const quotaPct = u.storage_quota_bytes > 0 ? ((u.storage_used_bytes / u.storage_quota_bytes) * 100) : 0;
      const quotaColor = quotaPct > 90 ? 'red' : quotaPct > 70 ? 'orange' : 'green';
      const quotaText = u.storage_quota_bytes > 0 ? formatBytes(u.storage_used_bytes) + ' / ' + formatBytes(u.storage_quota_bytes) : formatBytes(u.storage_used_bytes) + ' / ∞';
      const isSelf = u.id === currentAdminId;
      return '<tr>' +
        '<td><div class="user-info"><span class="user-name">' + u.username + (isSelf ? ' <span class="user-self-badge">(you)</span>' : '') + '</span><span class="user-email">' + u.email + '</span></div></td>' +
        '<td><span class="badge badge-' + u.role + '">' + (u.role === 'admin' ? '<i class="fas fa-shield-alt badge-admin-icon-small"></i> ' : '') + u.role + '</span></td>' +
        '<td><span class="badge badge-' + (u.active ? 'active' : 'inactive') + '">' + (u.active ? 'Active' : 'Inactive') + '</span></td>' +
        '<td><div class="quota-bar"><div class="progress-bar quota-progress-fixed"><div class="progress-fill ' + quotaColor + '" style="width:' + Math.min(quotaPct, 100) + '%"></div></div><span class="quota-text">' + quotaText + '</span></div></td>' +
        '<td class="user-last-login-cell">' + timeAgo(u.last_login_at) + '</td>' +
        '<td><div class="actions-row">' +
          '<button class="btn btn-sm btn-secondary" onclick="openQuotaModal(\'' + u.id + '\',\'' + u.username + '\',' + u.storage_quota_bytes + ')" title="Edit quota"><i class="fas fa-box"></i></button>' +
          '<button class="btn btn-sm btn-secondary" onclick="openResetPasswordModal(\'' + u.id + '\',\'' + u.username + '\')" title="Reset password"><i class="fas fa-key"></i></button>' +
          '<button class="btn btn-sm btn-secondary" onclick="toggleRole(\'' + u.id + '\',\'' + u.role + '\')" title="Toggle role"' + (isSelf ? ' disabled' : '') + '><i class="fas fa-' + (u.role === 'admin' ? 'user' : 'crown') + '"></i></button>' +
          '<button class="btn btn-sm ' + (u.active ? 'btn-danger' : 'btn-success') + '" onclick="toggleActive(\'' + u.id + '\',' + u.active + ')" title="' + (u.active ? 'Deactivate' : 'Activate') + '"' + (isSelf && u.active ? ' disabled' : '') + '><i class="fas fa-' + (u.active ? 'ban' : 'check') + '"></i></button>' +
          '<button class="btn btn-sm btn-danger" onclick="deleteUser(\'' + u.id + '\',\'' + u.username + '\')" title="Delete"' + (isSelf ? ' disabled' : '') + '><i class="fas fa-trash-alt"></i></button>' +
        '</div></td></tr>';
    }).join('');

    document.getElementById('users-info').textContent = 'Showing ' + (usersPage * PAGE_SIZE + 1) + '-' + Math.min((usersPage + 1) * PAGE_SIZE, totalUsers) + ' of ' + totalUsers;
    document.getElementById('prev-btn').disabled = usersPage === 0;
    document.getElementById('next-btn').disabled = (usersPage + 1) * PAGE_SIZE >= totalUsers;
  } catch (e) {
    tbody.innerHTML = '<tr><td colspan="6" class="table-status-error"><i class="fas fa-exclamation-circle"></i> Error: ' + e.message + '</td></tr>';
  }
}

function prevPage() { if (usersPage > 0) { usersPage--; loadUsers(); } }
function nextPage() { if ((usersPage + 1) * PAGE_SIZE < totalUsers) { usersPage++; loadUsers(); } }

async function toggleRole(userId, currentRole) {
  const newRole = currentRole === 'admin' ? 'user' : 'admin';
  if (!confirm('Change role to ' + newRole + '?')) return;
  try {
    const resp = await fetch(API + '/admin/users/' + userId + '/role', {
      method: 'PUT', headers: headers(), body: JSON.stringify({ role: newRole })
    });
    if (resp.ok) loadUsers(); else { const e = await resp.json(); alert(e.message || 'Failed'); }
  } catch (e) { alert('Error: ' + e.message); }
}

async function toggleActive(userId, currentActive) {
  const action = currentActive ? 'deactivate' : 'activate';
  if (!confirm('Are you sure you want to ' + action + ' this user?')) return;
  try {
    const resp = await fetch(API + '/admin/users/' + userId + '/active', {
      method: 'PUT', headers: headers(), body: JSON.stringify({ active: !currentActive })
    });
    if (resp.ok) loadUsers(); else { const e = await resp.json(); alert(e.message || 'Failed'); }
  } catch (e) { alert('Error: ' + e.message); }
}

async function deleteUser(userId, username) {
  if (!confirm('DELETE user "' + username + '"? This cannot be undone!')) return;
  try {
    const resp = await fetch(API + '/admin/users/' + userId, { method: 'DELETE', headers: headers() });
    if (resp.ok) { loadUsers(); loadDashboard(); } else { const e = await resp.json(); alert(e.message || 'Failed'); }
  } catch (e) { alert('Error: ' + e.message); }
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
function closeQuotaModal() { hideElement('quota-modal'); }

async function saveQuota() {
  const val = parseFloat(document.getElementById('qm-value').value) || 0;
  const unit = parseInt(document.getElementById('qm-unit').value);
  const bytes = Math.round(val * unit);
  try {
    const resp = await fetch(API + '/admin/users/' + quotaUserId + '/quota', {
      method: 'PUT', headers: headers(), body: JSON.stringify({ quota_bytes: bytes })
    });
    if (resp.ok) { closeQuotaModal(); loadUsers(); loadDashboard(); }
    else { const e = await resp.json(); alert(e.message || 'Failed'); }
  } catch (e) { alert('Error: ' + e.message); }
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
function closeCreateUserModal() { hideElement('create-user-modal'); }

async function submitCreateUser() {
  const username = document.getElementById('cu-username').value.trim();
  const password = document.getElementById('cu-password').value;
  const email = document.getElementById('cu-email').value.trim() || null;
  const role = document.getElementById('cu-role').value;
  const quotaVal = parseFloat(document.getElementById('cu-quota-value').value) || 0;
  const quotaUnit = parseInt(document.getElementById('cu-quota-unit').value);
  const quotaBytes = Math.round(quotaVal * quotaUnit);

  const errorEl = document.getElementById('cu-error');
  if (username.length < 3) { errorEl.textContent = 'Username must be at least 3 characters'; errorEl.className = 'alert alert-error'; return; }
  if (password.length < 8) { errorEl.textContent = 'Password must be at least 8 characters'; errorEl.className = 'alert alert-error'; return; }

  const btn = document.getElementById('cu-submit');
  btn.disabled = true; btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Creating…';
  try {
    const resp = await fetch(API + '/admin/users', {
      method: 'POST', headers: headers(),
      body: JSON.stringify({ username, password, email, role, quota_bytes: quotaBytes })
    });
    if (resp.ok) {
      closeCreateUserModal();
      loadUsers();
      loadDashboard();
    } else {
      const e = await resp.json().catch(() => ({}));
      errorEl.textContent = e.message || 'Failed to create user';
      errorEl.className = 'alert alert-error';
    }
  } catch (e) {
    errorEl.textContent = 'Network error: ' + e.message;
    errorEl.className = 'alert alert-error';
  }
  btn.disabled = false; btn.innerHTML = '<i class="fas fa-user-plus"></i> Create';
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
function closeResetPasswordModal() { hideElement('reset-pw-modal'); }

async function submitResetPassword() {
  const password = document.getElementById('rp-password').value;
  const errorEl = document.getElementById('rp-error');
  if (password.length < 8) { errorEl.textContent = 'Password must be at least 8 characters'; errorEl.className = 'alert alert-error'; return; }

  const btn = document.getElementById('rp-submit');
  btn.disabled = true; btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Resetting…';
  try {
    const resp = await fetch(API + '/admin/users/' + resetPwUserId + '/password', {
      method: 'PUT', headers: headers(),
      body: JSON.stringify({ new_password: password })
    });
    if (resp.ok) { closeResetPasswordModal(); }
    else { const e = await resp.json().catch(() => ({})); errorEl.textContent = e.message || 'Failed'; errorEl.className = 'alert alert-error'; }
  } catch (e) { errorEl.textContent = 'Error: ' + e.message; errorEl.className = 'alert alert-error'; }
  btn.disabled = false; btn.innerHTML = '<i class="fas fa-save"></i> Reset';
}

async function toggleRegistration(enabled) {
  if (enabled) hideElement('registration-warning');
  else showElement('registration-warning', 'flex');
  try {
    const resp = await fetch(API + '/admin/settings/registration', {
      method: 'PUT', headers: headers(),
      body: JSON.stringify({ registration_enabled: enabled })
    });
    if (!resp.ok) {
      document.getElementById('ds-registration').checked = !enabled;
      if (!enabled) showElement('registration-warning', 'flex');
      else hideElement('registration-warning');
      const e = await resp.json().catch(() => ({}));
      alert(e.message || 'Failed to update registration setting');
    }
  } catch (e) {
    document.getElementById('ds-registration').checked = !enabled;
    if (!enabled) showElement('registration-warning', 'flex');
    else hideElement('registration-warning');
    alert('Error: ' + e.message);
  }
}

document.getElementById('oidc-enabled').addEventListener('change', function() {
  if (this.checked) showElement('oidc-form');
  else hideElement('oidc-form');
});
document.getElementById('disable-password').addEventListener('change', function() {
  if (this.checked) showElement('password-warning', 'flex');
  else hideElement('password-warning');
});

function showOidcStatus(msg, type) {
  const el = document.getElementById('oidc-status');
  el.textContent = msg;
  el.className = 'alert alert-' + type;
}

function copyCallback() {
  const text = document.getElementById('callback-url').textContent;
  navigator.clipboard.writeText(text);
}

async function testConnection() {
  const url = document.getElementById('issuer-url').value.trim();
  if (!url) { showOidcStatus('Enter an Issuer URL first', 'error'); return; }
  const btn = document.getElementById('discover-btn');
  btn.disabled = true; btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Discovering…';
  const resultDiv = document.getElementById('discovery-result');
  try {
    const resp = await fetch(API + '/admin/settings/oidc/test', { method: 'POST', headers: headers(), body: JSON.stringify({ issuer_url: url }) });
    const r = await resp.json();
    if (r.success) {
      resultDiv.innerHTML = '<div class="discovery-result ok"><strong><i class="fas fa-check-circle"></i> ' + r.message + '</strong><dl><dt>Issuer</dt><dd>' + (r.issuer||'—') + '</dd><dt>Auth Endpoint</dt><dd>' + (r.authorization_endpoint||'—') + '</dd></dl></div>';
      if (!document.getElementById('provider-name').value && r.provider_name_suggestion) document.getElementById('provider-name').value = r.provider_name_suggestion;
    } else {
      resultDiv.innerHTML = '<div class="discovery-result fail"><strong><i class="fas fa-times-circle"></i> ' + r.message + '</strong></div>';
    }
  } catch (e) { resultDiv.innerHTML = '<div class="discovery-result fail"><i class="fas fa-times-circle"></i> Error: ' + e.message + '</div>'; }
  btn.disabled = false; btn.innerHTML = '<i class="fas fa-search"></i> Auto-discover';
}

async function saveOidcSettings() {
  const btn = document.getElementById('save-btn');
  btn.disabled = true; btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Saving…';
  const body = {
    enabled: document.getElementById('oidc-enabled').checked,
    issuer_url: document.getElementById('issuer-url').value.trim(),
    client_id: document.getElementById('client-id').value.trim(),
    client_secret: document.getElementById('client-secret').value || null,
    scopes: document.getElementById('scopes').value.trim() || null,
    auto_provision: document.getElementById('auto-provision').checked,
    admin_groups: document.getElementById('admin-groups').value.trim() || null,
    disable_password_login: document.getElementById('disable-password').checked,
    provider_name: document.getElementById('provider-name').value.trim() || null,
  };
  try {
    const resp = await fetch(API + '/admin/settings/oidc', { method: 'PUT', headers: headers(), body: JSON.stringify(body) });
    if (resp.ok) { showOidcStatus('Settings saved — OIDC is now ' + (body.enabled ? 'active' : 'disabled'), 'success'); loadDashboard(); }
    else { const e = await resp.json().catch(()=>({})); showOidcStatus('Error: ' + (e.message || resp.statusText), 'error'); }
  } catch (e) { showOidcStatus('Network error: ' + e.message, 'error'); }
  btn.disabled = false; btn.innerHTML = '<i class="fas fa-save"></i> Save';
}

async function init() {
  if (!token) { showAccessDenied(); return; }
  try {
    const me = await fetch(API + '/auth/me', { headers: headers() });
    if (!me.ok) { showAccessDenied(); return; }
    const user = await me.json();
    if (user.role !== 'admin') { showAccessDenied(); return; }
    currentAdminId = user.id;

    const oidcResp = await fetch(API + '/admin/settings/oidc', { headers: headers() });
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
      (s.env_overrides || []).forEach(field => {
        const badge = document.getElementById('badge-' + field);
        if (badge) badge.innerHTML = '<span class="badge badge-env">ENV</span>';
      });
    }

    await loadDashboard();
    hideElement('loading');
    showElement('main-content');
  } catch (e) { console.error(e); showAccessDenied(); }
}

function showAccessDenied() {
  hideElement('loading');
  showElement('access-denied');
}

init();
