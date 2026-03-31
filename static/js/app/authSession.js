/**
 * Authentication/session bootstrap and home-folder resolution
 */

async function refreshUserData() {
    const USER_DATA_KEY = 'oxicloud_user';

    try {
        console.log('Fetching /api/auth/me (cookie-based)...');
        const response = await fetch('/api/auth/me', {
            method: 'GET',
            credentials: 'same-origin'
        });

        console.log('/api/auth/me response status:', response.status);

        if (!response.ok) {
            console.warn('Failed to fetch user data:', response.status);
            return null;
        }

        const userData = await response.json();
        console.log('Refreshed user data from server:', userData);
        console.log('Storage from server: used=', userData.storage_used_bytes, 'quota=', userData.storage_quota_bytes);

        localStorage.setItem(USER_DATA_KEY, JSON.stringify(userData));
        window.updateStorageUsageDisplay(userData);

        return userData;
    } catch (error) {
        console.error('Error refreshing user data:', error);
        return null;
    }
}

async function checkAuthentication() {
    try {
        const USER_DATA_KEY = 'oxicloud_user';

        const urlParams = new URLSearchParams(window.location.search);
        const oidcCode = urlParams.get('oidc_code');

        if (oidcCode) {
            console.log('OIDC exchange code detected, exchanging for tokens...');
            try {
                const exchangeResponse = await fetch('/api/auth/oidc/exchange', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json', ...getCsrfHeaders() },
                    body: JSON.stringify({ code: oidcCode }),
                    credentials: 'same-origin'
                });

                if (!exchangeResponse.ok) {
                    const errText = await exchangeResponse.text();
                    console.error('OIDC token exchange failed:', exchangeResponse.status, errText);
                    window.location.href = '/login?source=oidc_error';
                    return;
                }

                const data = await exchangeResponse.json();
                console.log('OIDC token exchange successful');

                // Tokens are now in HttpOnly cookies set by the server.
                if (data.user) {
                    localStorage.setItem(USER_DATA_KEY, JSON.stringify(data.user));
                }

                window.history.replaceState({}, document.title, '/');
                window.location.reload();
                return;
            } catch (err) {
                console.error('OIDC exchange error:', err);
                window.location.href = '/login?source=oidc_error';
                return;
            }
        }

        // Check session validity by calling /api/auth/me (cookie auto-sent)
        console.log('Checking session via /api/auth/me...');

        const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
        if (userData.username) {
            // We have cached user data — render immediately, refresh in background
            const userInitials = userData.username.substring(0, 2).toUpperCase();
            document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => {
                el.textContent = userInitials;
            });
            const menuName = document.getElementById('user-menu-name');
            const menuEmail = document.getElementById('user-menu-email');
            if (menuName) menuName.textContent = userData.username;
            if (menuEmail) menuEmail.textContent = userData.email || '';

            window.updateStorageUsageDisplay(userData);

            // Validate session BEFORE loading files to avoid 401 race condition
            const freshData = await refreshUserData();
            if (freshData) {
                console.log('Storage usage updated from server');
            } else {
                // Session expired — try silent refresh first
                console.warn('Session may have expired, trying refresh...');
                try {
                    const r = await fetch('/api/auth/refresh', {
                        method: 'POST',
                        credentials: 'same-origin',
                        headers: { 'Content-Type': 'application/json', ...getCsrfHeaders() },
                        body: '{}'
                    });
                    if (r.ok) {
                        await refreshUserData();
                    } else {
                        localStorage.removeItem(USER_DATA_KEY);
                        window.location.href = '/login?source=session_expired';
                        return;
                    }
                } catch (err) {
                    localStorage.removeItem(USER_DATA_KEY);
                    window.location.href = '/login?source=session_expired';
                    return;
                }
            }

            await resolveHomeFolder();
            window.loadFiles();
        } else {
            // No cached user data — must verify session from server
            console.log('No cached user data, fetching from server');
            try {
                const freshData = await refreshUserData();
                if (freshData && freshData.username) {
                    const userInitials = freshData.username.substring(0, 2).toUpperCase();
                    document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => el.textContent = userInitials);
                    window.updateStorageUsageDisplay(freshData);
                    resolveHomeFolder().then(() => window.loadFiles());
                } else {
                    console.warn('Could not retrieve user data, redirecting to login');
                    localStorage.removeItem(USER_DATA_KEY);
                    window.location.href = '/login?source=invalid_session';
                }
            } catch (err) {
                console.error('Failed to fetch user data:', err);
                localStorage.removeItem(USER_DATA_KEY);
                window.location.href = '/login?source=session_error';
            }
        }
    } catch (error) {
        console.error('Error during authentication check:', error);
        localStorage.removeItem('oxicloud_user');
        window.location.href = '/login?source=auth_error';
    }
}

async function resolveHomeFolder() {
    const app = window.app;

    if (app.userHomeFolderId) return;
    try {
        const response = await fetch('/api/folders', { credentials: 'same-origin' });
        if (!response.ok) {
            console.warn(`Could not fetch home folder: ${response.status}`);
            return;
        }
        const folders = await response.json();
        const folderList = Array.isArray(folders) ? folders : [];
        if (folderList.length > 0) {
            const home = folderList[0];
            app.userHomeFolderId = home.id;
            app.userHomeFolderName = home.name;
            app.currentPath = home.id;
            app.breadcrumbPath = [];
            window.ui.updateBreadcrumb();
            console.log(`Home folder resolved: ${home.name} (${home.id})`);
        } else {
            console.warn('No root folders found for user');
            app.currentPath = '';
            app.breadcrumbPath = [];
            window.ui.updateBreadcrumb();
        }
    } catch (error) {
        console.error('Error resolving home folder:', error);
        app.currentPath = '';
        app.breadcrumbPath = [];
        window.ui.updateBreadcrumb();
    }
}

window.refreshUserData = refreshUserData;
window.checkAuthentication = checkAuthentication;
window.resolveHomeFolder = resolveHomeFolder;
