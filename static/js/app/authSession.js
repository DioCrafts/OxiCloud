/**
 * Authentication/session bootstrap and home-folder resolution
 */

async function refreshUserData() {
    const TOKEN_KEY = 'oxicloud_token';
    const USER_DATA_KEY = 'oxicloud_user';

    const token = localStorage.getItem(TOKEN_KEY);
    console.log('refreshUserData called, token:', token ? token.substring(0, 20) + '...' : 'null');

    if (!token) {
        console.log('No valid token, skipping user data refresh');
        return null;
    }

    try {
        console.log('Fetching /api/auth/me...');
        const response = await fetch('/api/auth/me', {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
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
        const TOKEN_KEY = 'oxicloud_token';
        const REFRESH_TOKEN_KEY = 'oxicloud_refresh_token';
        const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
        const USER_DATA_KEY = 'oxicloud_user';

        const urlParams = new URLSearchParams(window.location.search);
        const oidcCode = urlParams.get('oidc_code');

        if (oidcCode) {
            console.log('OIDC exchange code detected, exchanging for tokens...');
            try {
                const exchangeResponse = await fetch('/api/auth/oidc/exchange', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ code: oidcCode })
                });

                if (!exchangeResponse.ok) {
                    const errText = await exchangeResponse.text();
                    console.error('OIDC token exchange failed:', exchangeResponse.status, errText);
                    window.location.href = '/login?source=oidc_error';
                    return;
                }

                const data = await exchangeResponse.json();
                console.log('OIDC token exchange successful');

                const token = data.access_token || data.token;
                const refreshToken = data.refresh_token || data.refreshToken;

                if (token) {
                    localStorage.setItem(TOKEN_KEY, token);
                    if (refreshToken) localStorage.setItem(REFRESH_TOKEN_KEY, refreshToken);

                    let parsedExpiry = false;
                    const tokenParts = token.split('.');
                    if (tokenParts.length === 3) {
                        try {
                            const payload = JSON.parse(atob(tokenParts[1]));
                            if (payload.exp) {
                                const expiryDate = new Date(payload.exp * 1000);
                                if (!isNaN(expiryDate.getTime())) {
                                    localStorage.setItem(TOKEN_EXPIRY_KEY, expiryDate.toISOString());
                                    parsedExpiry = true;
                                }
                            }
                        } catch (e) {
                            console.error('Error parsing JWT:', e);
                        }
                    }
                    if (!parsedExpiry) {
                        const expiry = new Date();
                        expiry.setDate(expiry.getDate() + 30);
                        localStorage.setItem(TOKEN_EXPIRY_KEY, expiry.toISOString());
                    }

                    if (data.user) {
                        localStorage.setItem(USER_DATA_KEY, JSON.stringify(data.user));
                    }

                    window.history.replaceState({}, document.title, '/');
                    window.location.reload();
                    return;
                }
            } catch (err) {
                console.error('OIDC exchange error:', err);
                window.location.href = '/login?source=oidc_error';
                return;
            }
        }

        const token = localStorage.getItem(TOKEN_KEY);

        if (!token) {
            console.log('No token found, redirecting to login');
            window.location.href = '/login?source=app';
            return;
        }

        console.log('Token found, proceeding with app initialization');

        const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
        if (userData.username) {
            const userInitials = userData.username.substring(0, 2).toUpperCase();
            document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => {
                el.textContent = userInitials;
            });
            const menuName = document.getElementById('user-menu-name');
            const menuEmail = document.getElementById('user-menu-email');
            if (menuName) menuName.textContent = userData.username;
            if (menuEmail) menuEmail.textContent = userData.email || '';

            window.updateStorageUsageDisplay(userData);

            refreshUserData().then(freshData => {
                if (freshData) {
                    console.log('Storage usage updated from server');
                }
            }).catch(err => {
                console.warn('Could not refresh user data:', err);
            });

            resolveHomeFolder().then(() => window.loadFiles());
        } else {
            console.log('No user data, attempting to fetch from server');
            try {
                const freshData = await refreshUserData();
                if (freshData && freshData.username) {
                    const userInitials = freshData.username.substring(0, 2).toUpperCase();
                    document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => el.textContent = userInitials);
                    window.updateStorageUsageDisplay(freshData);
                    resolveHomeFolder().then(() => window.loadFiles());
                } else {
                    console.warn('Could not retrieve user data, redirecting to login');
                    localStorage.removeItem(TOKEN_KEY);
                    localStorage.removeItem(REFRESH_TOKEN_KEY);
                    localStorage.removeItem(TOKEN_EXPIRY_KEY);
                    localStorage.removeItem(USER_DATA_KEY);
                    window.location.href = '/login?source=invalid_session';
                }
            } catch (err) {
                console.error('Failed to fetch user data:', err);
                localStorage.removeItem(TOKEN_KEY);
                localStorage.removeItem(REFRESH_TOKEN_KEY);
                localStorage.removeItem(TOKEN_EXPIRY_KEY);
                localStorage.removeItem(USER_DATA_KEY);
                window.location.href = '/login?source=session_error';
            }
        }
    } catch (error) {
        console.error('Error during authentication check:', error);
        localStorage.removeItem('oxicloud_token');
        localStorage.removeItem('oxicloud_refresh_token');
        localStorage.removeItem('oxicloud_token_expiry');
        localStorage.removeItem('oxicloud_user');
        window.location.href = '/login?source=auth_error';
    }
}

async function resolveHomeFolder() {
    const app = window.app;

    if (app.userHomeFolderId) return;
    try {
        const token = localStorage.getItem('oxicloud_token');
        const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
        const response = await fetch('/api/folders', { headers });
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
            window.ui.updateBreadcrumb(home.name);
            console.log(`Home folder resolved: ${home.name} (${home.id})`);
        } else {
            console.warn('No root folders found for user');
            app.currentPath = '';
            window.ui.updateBreadcrumb('');
        }
    } catch (error) {
        console.error('Error resolving home folder:', error);
        app.currentPath = '';
        window.ui.updateBreadcrumb('');
    }
}

window.refreshUserData = refreshUserData;
window.checkAuthentication = checkAuthentication;
window.resolveHomeFolder = resolveHomeFolder;
