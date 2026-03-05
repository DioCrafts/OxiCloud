// Extract token from URL path and set form action
var pathParts = window.location.pathname.split('/');
var token = pathParts[pathParts.length - 1];
// Validate token is hex-only to prevent injection
if (!/^[0-9a-fA-F]+$/.test(token)) {
    document.body.innerHTML = '<p>Invalid session token.</p>';
    throw new Error('Invalid token format');
}
document.getElementById('login-flow-form').action = '/login/v2/flow/' + token;

// Check if OIDC is available and configure SSO button
(async function() {
    try {
        var resp = await fetch('/api/auth/oidc/providers');
        if (!resp.ok) return;
        var info = await resp.json();
        if (!info.enabled) return;

        // Show OIDC section
        document.getElementById('oidc-section').classList.remove('hidden');

        // Update button text with provider name
        var btn = document.getElementById('oidc-button');
        btn.textContent = 'Sign in with ' + (info.provider_name || 'SSO');

        // If password login is disabled, hide the password form
        if (!info.password_login_enabled) {
            document.getElementById('login-flow-form').style.display = 'none';
        }

        // SSO button redirects to the OIDC flow for this NC token
        btn.addEventListener('click', function() {
            window.location.href = '/login/v2/flow/' + token + '/oidc';
        });
    } catch(e) {
        // OIDC not available — silently keep password-only mode
    }
})();
