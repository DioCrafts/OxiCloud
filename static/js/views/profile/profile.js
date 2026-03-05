const API = '/api';

function headers() {
  return { 'Content-Type': 'application/json', ...getCsrfHeaders() };
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
  if (secs < 3600) return Math.floor(secs/60) + ' min ago';
  if (secs < 86400) return Math.floor(secs/3600) + 'h ago';
  if (secs < 2592000) return Math.floor(secs/86400) + ' days ago';
  return d.toLocaleDateString();
}

async function init() {
  try {
    const resp = await fetch(API + '/auth/me', { headers: headers(), credentials: 'same-origin' });
    if (!resp.ok) { showError(); return; }
    const user = await resp.json();

    const initials = (user.username || '?').substring(0, 2).toUpperCase();
    document.getElementById('p-avatar').textContent = initials;
    document.getElementById('p-username').textContent = user.username;
    document.getElementById('p-email').textContent = user.email || '';

    const badge = document.getElementById('p-role-badge');
    if (user.role === 'admin') {
      badge.className = 'role-badge role-badge-admin';
      badge.innerHTML = '<i class="fas fa-shield-alt"></i> Administrator';
    } else {
      badge.className = 'role-badge role-badge-user';
      badge.innerHTML = '<i class="fas fa-user"></i> User';
    }

    document.getElementById('p-detail-username').textContent = user.username;
    document.getElementById('p-detail-email').textContent = user.email || '—';
    document.getElementById('p-detail-role').textContent = user.role === 'admin' ? 'Administrator' : 'User';
    document.getElementById('p-detail-login').textContent = timeAgo(user.last_login_at);

    const used = user.storage_used_bytes || 0;
    const quota = user.storage_quota_bytes || 0;
    const pct = quota > 0 ? Math.min(Math.round((used / quota) * 100), 100) : 0;

    document.getElementById('p-storage-used').textContent = formatBytes(used);
    document.getElementById('p-storage-quota').textContent = quota > 0 ? formatBytes(quota) : '∞';
    document.getElementById('p-storage-pct').textContent = quota > 0 ? pct + '%' : '—';

    const bar = document.getElementById('p-storage-bar');
    bar.style.width = pct + '%';
    bar.className = 'storage-fill ' + (pct > 90 ? 'red' : pct > 70 ? 'orange' : 'green');
    document.getElementById('p-storage-text').textContent = formatBytes(used) + ' / ' + (quota > 0 ? formatBytes(quota) : 'Unlimited');

    if (user.auth_provider && user.auth_provider !== 'local') {
      document.getElementById('password-section').style.display = 'none';
    }

    loadAppPasswords();

    try {
      const oidcResp = await fetch(API + '/auth/oidc/providers', { credentials: 'same-origin' });
      if (oidcResp.ok) {
        const oidcInfo = await oidcResp.json();
        if (!oidcInfo.password_login_enabled) {
          document.getElementById('password-section').style.display = 'none';
        }
      }
    } catch (oidcErr) {
    }

    document.getElementById('loading').style.display = 'none';
    document.getElementById('main-content').style.display = 'block';
  } catch (e) {
    console.error(e);
    showError();
  }
}

function showError() {
  document.getElementById('loading').style.display = 'none';
  document.getElementById('auth-error').style.display = 'block';
}

async function changePassword(e) {
  e.preventDefault();
  const currentPw = document.getElementById('current-password').value;
  const newPw = document.getElementById('new-password').value;
  const confirmPw = document.getElementById('confirm-password').value;
  const statusEl = document.getElementById('pw-status');

  if (newPw !== confirmPw) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> Passwords do not match</div>';
    return false;
  }

  if (newPw.length < 8) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> Password must be at least 8 characters</div>';
    return false;
  }

  const btn = document.getElementById('pw-submit');
  btn.disabled = true;
  btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Updating…';

  try {
    const resp = await fetch(API + '/auth/change-password', {
      method: 'PUT',
      headers: headers(),
      credentials: 'same-origin',
      body: JSON.stringify({ current_password: currentPw, new_password: newPw })
    });

    if (resp.ok) {
      statusEl.innerHTML = '<div class="alert alert-success"><i class="fas fa-check-circle"></i> Password updated successfully</div>';
      document.getElementById('password-form').reset();
    } else {
      const err = await resp.json().catch(() => ({}));
      statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + escapeHtml(err.message || 'Failed to change password') + '</div>';
    }
  } catch (err) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> Network error: ' + escapeHtml(err.message) + '</div>';
  }

  btn.disabled = false;
  btn.innerHTML = '<i class="fas fa-save"></i> Update Password';
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
  lastUsed.textContent = pw.last_used_at ? timeAgo(pw.last_used_at) : 'Never';
  const actions = document.createElement('td');
  const btn = document.createElement('button');
  btn.className = 'btn btn-danger-sm';
  btn.innerHTML = '<i class="fas fa-trash"></i>';
  btn.title = 'Revoke';
  btn.onclick = function () { revokeAppPassword(pw.id, pw.label); };
  actions.appendChild(btn);
  tr.append(label, created, lastUsed, actions);
  return tr;
}

async function loadAppPasswords() {
  try {
    const resp = await fetch(API + '/auth/app-passwords', { headers: headers() });
    if (!resp.ok) {
      document.getElementById('app-passwords-section').style.display = 'none';
      return;
    }
    const passwords = await resp.json();
    const userPws = passwords.filter(function (pw) { return !isAutoPassword(pw); });
    const autoPws = passwords.filter(isAutoPassword);

    // User-created passwords
    const tbody = document.getElementById('app-pw-tbody');
    const table = document.getElementById('app-pw-table');
    const empty = document.getElementById('app-pw-empty');
    tbody.innerHTML = '';
    if (userPws.length === 0) {
      table.style.display = 'none';
      empty.style.display = 'block';
    } else {
      table.style.display = '';
      empty.style.display = 'none';
      for (const pw of userPws) tbody.appendChild(renderPwRow(pw));
    }

    // Auto-generated (client session) passwords
    const autoSection = document.getElementById('app-pw-auto-section');
    if (autoPws.length === 0) {
      autoSection.style.display = 'none';
    } else {
      autoSection.style.display = '';
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
  const open = body.style.display === 'none';
  body.style.display = open ? '' : 'none';
  chevron.className = open ? 'fas fa-chevron-down' : 'fas fa-chevron-right';
}

async function createAppPassword() {
  const labelInput = document.getElementById('app-pw-label');
  const label = labelInput.value.trim();
  const statusEl = document.getElementById('app-pw-status');
  const btn = document.getElementById('app-pw-generate');

  if (!label) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> Please enter a label</div>';
    return;
  }

  btn.disabled = true;
  btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Generating…';
  statusEl.innerHTML = '';

  try {
    const resp = await fetch(API + '/auth/app-passwords', {
      method: 'POST',
      headers: headers(),
      body: JSON.stringify({ label: label })
    });
    if (!resp.ok) {
      const err = await resp.json().catch(() => ({}));
      statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + (err.message || 'Failed to create app password') + '</div>';
      return;
    }
    const result = await resp.json();
    document.getElementById('app-pw-created-label').textContent = result.label;
    document.getElementById('app-pw-created-password').textContent = result.password;
    document.getElementById('app-pw-created').style.display = 'block';
    labelInput.value = '';
    loadAppPasswords();
  } catch (err) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + err.message + '</div>';
  } finally {
    btn.disabled = false;
    btn.innerHTML = '<i class="fas fa-plus"></i> Generate';
  }
}

function copyAppPassword() {
  const pw = document.getElementById('app-pw-created-password').textContent;
  navigator.clipboard.writeText(pw).then(function () {
    const btn = document.querySelector('.btn-copy');
    btn.innerHTML = '<i class="fas fa-check"></i>';
    setTimeout(function () { btn.innerHTML = '<i class="fas fa-copy"></i>'; }, 1500);
  });
}

async function revokeAppPassword(id, label) {
  if (!confirm('Revoke app password "' + label + '"? Clients using this password will stop working.')) return;
  try {
    const resp = await fetch(API + '/auth/app-passwords/' + encodeURIComponent(id), {
      method: 'DELETE',
      headers: headers()
    });
    if (resp.ok || resp.status === 204) {
      document.getElementById('app-pw-created').style.display = 'none';
      loadAppPasswords();
    } else {
      const err = await resp.json().catch(() => ({}));
      alert(err.message || 'Failed to revoke app password');
    }
  } catch (err) {
    alert('Network error: ' + err.message);
  }
}

init();

/* Wire up form handler (replaces inline onsubmit) */
document.getElementById('password-form').addEventListener('submit', changePassword);
