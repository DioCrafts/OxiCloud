const API = '/api';
const token = localStorage.getItem('oxicloud_token') || localStorage.getItem('token') || localStorage.getItem('access_token');

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
  if (secs < 3600) return Math.floor(secs/60) + ' min ago';
  if (secs < 86400) return Math.floor(secs/3600) + 'h ago';
  if (secs < 2592000) return Math.floor(secs/86400) + ' days ago';
  return d.toLocaleDateString();
}

async function init() {
  if (!token) { showError(); return; }
  try {
    const resp = await fetch(API + '/auth/me', { headers: headers() });
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

    try {
      const oidcResp = await fetch(API + '/auth/oidc/providers');
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
      body: JSON.stringify({ current_password: currentPw, new_password: newPw })
    });

    if (resp.ok) {
      statusEl.innerHTML = '<div class="alert alert-success"><i class="fas fa-check-circle"></i> Password updated successfully</div>';
      document.getElementById('password-form').reset();
    } else {
      const err = await resp.json().catch(() => ({}));
      statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + (err.message || 'Failed to change password') + '</div>';
    }
  } catch (err) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> Network error: ' + err.message + '</div>';
  }

  btn.disabled = false;
  btn.innerHTML = '<i class="fas fa-save"></i> Update Password';
  return false;
}

init();
