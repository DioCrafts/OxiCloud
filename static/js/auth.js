/**
 * OxiCloud Authentication JavaScript
 * Handles login, registration, and admin setup
 */

// API endpoints
const API_URL = '/api/auth';
const LOGIN_ENDPOINT = `${API_URL}/login`;
const REGISTER_ENDPOINT = `${API_URL}/register`;
const ME_ENDPOINT = `${API_URL}/me`;
const REFRESH_ENDPOINT = `${API_URL}/refresh`;

// Storage keys
const TOKEN_KEY = 'oxicloud_token';
const REFRESH_TOKEN_KEY = 'oxicloud_refresh_token';
const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
const USER_DATA_KEY = 'oxicloud_user';

// DOM elements
const loginPanel = document.getElementById('login-panel');
const registerPanel = document.getElementById('register-panel');
const adminSetupPanel = document.getElementById('admin-setup-panel');

const loginForm = document.getElementById('login-form');
const registerForm = document.getElementById('register-form');
const adminSetupForm = document.getElementById('admin-setup-form');

const loginError = document.getElementById('login-error');
const registerError = document.getElementById('register-error');
const registerSuccess = document.getElementById('register-success');
const adminSetupError = document.getElementById('admin-setup-error');

// Panel toggles
const showRegisterBtn = document.getElementById('show-register');
if (showRegisterBtn) {
    showRegisterBtn.addEventListener('click', () => {
        loginPanel.style.display = 'none';
        registerPanel.style.display = 'block';
        adminSetupPanel.style.display = 'none';
    });
}

const showLoginBtn = document.getElementById('show-login');
if (showLoginBtn) {
    showLoginBtn.addEventListener('click', () => {
        loginPanel.style.display = 'block';
        registerPanel.style.display = 'none';
        adminSetupPanel.style.display = 'none';
    });
}

const showAdminSetupBtn = document.getElementById('show-admin-setup');
if (showAdminSetupBtn) {
    showAdminSetupBtn.addEventListener('click', () => {
        loginPanel.style.display = 'none';
        registerPanel.style.display = 'none';
        adminSetupPanel.style.display = 'block';
    });
}

const backToLoginBtn = document.getElementById('back-to-login');
if (backToLoginBtn) {
    backToLoginBtn.addEventListener('click', () => {
        loginPanel.style.display = 'block';
        registerPanel.style.display = 'none';
        adminSetupPanel.style.display = 'none';
    });
}

// Check if we already have a valid token
document.addEventListener('DOMContentLoaded', async () => {
    try {
        const tokenExpiry = localStorage.getItem(TOKEN_EXPIRY_KEY);
        if (tokenExpiry && new Date(tokenExpiry) > new Date()) {
            // Token still valid, redirect to main app
            redirectToMainApp();
            return;
        }
        
        // Token expired, try to refresh
        const refreshToken = localStorage.getItem(REFRESH_TOKEN_KEY);
        if (refreshToken) {
            try {
                await refreshAuthToken(refreshToken);
                redirectToMainApp();
            } catch (error) {
                // Refresh failed, continue with login page
                console.log('Token refresh failed, user needs to login again');
            }
        }

        // Check if admin account exists (customize this as needed)
        const isFirstRun = await checkFirstRun();
        if (isFirstRun) {
            loginPanel.style.display = 'none';
            registerPanel.style.display = 'none';
            adminSetupPanel.style.display = 'block';
        }
    } catch (error) {
        console.error('Authentication check failed:', error);
    }
});

// Login form submission
if (loginForm) {
    loginForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        // Clear previous errors
        if (loginError) {
            loginError.style.display = 'none';
        }
        
        const usernameInput = document.getElementById('login-username');
        const passwordInput = document.getElementById('login-password');
        const username = usernameInput ? usernameInput.value : '';
        const password = passwordInput ? passwordInput.value : '';
    
        try {
            const data = await login(username, password);
            
            // Store auth data
            localStorage.setItem(TOKEN_KEY, data.token); // Nombre correcto del campo en la respuesta
            localStorage.setItem(REFRESH_TOKEN_KEY, data.refreshToken);
            
            // Extraer fecha de expiración desde el token JWT
            const tokenParts = data.token.split('.');
            if (tokenParts.length === 3) {
                try {
                    const payload = JSON.parse(atob(tokenParts[1]));
                    if (payload.exp) {
                        // payload.exp está en segundos desde epoch
                        const expiryDate = new Date(payload.exp * 1000);
                        localStorage.setItem(TOKEN_EXPIRY_KEY, expiryDate.toISOString());
                    } else {
                        // Si no hay exp, establecer un valor predeterminado (1 hora)
                        const expiryTime = new Date();
                        expiryTime.setHours(expiryTime.getHours() + 1);
                        localStorage.setItem(TOKEN_EXPIRY_KEY, expiryTime.toISOString());
                    }
                } catch (e) {
                    console.error('Error parsing JWT token:', e);
                    // Valor predeterminado en caso de error
                    const expiryTime = new Date();
                    expiryTime.setHours(expiryTime.getHours() + 1);
                    localStorage.setItem(TOKEN_EXPIRY_KEY, expiryTime.toISOString());
                }
            } else {
                // Token mal formado, establecer tiempo predeterminado
                const expiryTime = new Date();
                expiryTime.setHours(expiryTime.getHours() + 1);
                localStorage.setItem(TOKEN_EXPIRY_KEY, expiryTime.toISOString());
            }
            
            // Fetch and store user data
            // Usamos el token que acabamos de almacenar (en lugar de data.accessToken)
            const token = localStorage.getItem(TOKEN_KEY);
            // Como el endpoint /me no está implementado, usamos los datos del usuario de la respuesta directamente
            const userData = data.user || { id: '123', username: 'testuser', email: 'test@example.com', role: 'user' };
            localStorage.setItem(USER_DATA_KEY, JSON.stringify(userData));
            
            // Redirect to main app
            redirectToMainApp();
        } catch (error) {
            if (loginError) {
                loginError.textContent = error.message || 'Error al iniciar sesión';
                loginError.style.display = 'block';
            }
        }
    });
}

// Register form submission
if (registerForm) {
    registerForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        // Clear previous messages
        if (registerError) registerError.style.display = 'none';
        if (registerSuccess) registerSuccess.style.display = 'none';
        
        const usernameInput = document.getElementById('register-username');
        const emailInput = document.getElementById('register-email');
        const passwordInput = document.getElementById('register-password');
        const confirmPasswordInput = document.getElementById('register-password-confirm');
        
        const username = usernameInput ? usernameInput.value : '';
        const email = emailInput ? emailInput.value : '';
        const password = passwordInput ? passwordInput.value : '';
        const confirmPassword = confirmPasswordInput ? confirmPasswordInput.value : '';
    
        // Validate passwords match
        if (password !== confirmPassword) {
            if (registerError) {
                registerError.textContent = 'Las contraseñas no coinciden';
                registerError.style.display = 'block';
            }
            return;
        }
        
        try {
            const data = await register(username, email, password);
            
            // Show success message
            if (registerSuccess) {
                registerSuccess.textContent = '¡Cuenta creada con éxito! Puedes iniciar sesión ahora.';
                registerSuccess.style.display = 'block';
            }
            
            // Clear form
            registerForm.reset();
            
            // Switch to login panel after 2 seconds
            setTimeout(() => {
                if (loginPanel) loginPanel.style.display = 'block';
                if (registerPanel) registerPanel.style.display = 'none';
            }, 2000);
        } catch (error) {
            if (registerError) {
                registerError.textContent = error.message || 'Error al registrar cuenta';
                registerError.style.display = 'block';
            }
        }
    });
}

// Admin setup form submission
if (adminSetupForm) {
    adminSetupForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        // Clear previous errors
        if (adminSetupError) adminSetupError.style.display = 'none';
        
        const emailInput = document.getElementById('admin-email');
        const passwordInput = document.getElementById('admin-password');
        const email = emailInput ? emailInput.value : '';
        const password = passwordInput ? passwordInput.value : '';
        const confirmPasswordInput = document.getElementById('admin-password-confirm');
        const confirmPassword = confirmPasswordInput ? confirmPasswordInput.value : '';
    
        // Validate passwords match
        if (password !== confirmPassword) {
            if (adminSetupError) {
                adminSetupError.textContent = 'Las contraseñas no coinciden';
                adminSetupError.style.display = 'block';
            }
            return;
        }
    
        try {
            // Register admin account
            const data = await register('admin', email, password, 'admin');
            
            // Show success and switch to login
            alert('¡Cuenta de administrador creada con éxito! Ahora puedes iniciar sesión.');
            
            if (loginPanel) loginPanel.style.display = 'block';
            if (adminSetupPanel) adminSetupPanel.style.display = 'none';
        } catch (error) {
            if (adminSetupError) {
                adminSetupError.textContent = error.message || 'Error al crear cuenta de administrador';
                adminSetupError.style.display = 'block';
            }
        }
    });
}

// API Functions

/**
 * Login with username and password
 */
async function login(username, password) {
    try {
        const response = await fetch(LOGIN_ENDPOINT, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password })
        });
        
        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.error || 'Falló la autenticación');
        }
        
        return await response.json();
    } catch (error) {
        console.error('Login error:', error);
        throw error;
    }
}

/**
 * Register a new user
 */
async function register(username, email, password, role = 'user') {
    try {
        const response = await fetch(REGISTER_ENDPOINT, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, email, password, role })
        });
        
        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.error || 'Error en el registro');
        }
        
        return await response.json();
    } catch (error) {
        console.error('Registration error:', error);
        throw error;
    }
}

/**
 * Fetch current user data
 */
async function fetchUserData(token) {
    try {
        const response = await fetch(ME_ENDPOINT, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${token}`
            }
        });
        
        if (!response.ok) {
            throw new Error('Error al obtener datos del usuario');
        }
        
        return await response.json();
    } catch (error) {
        console.error('Error fetching user data:', error);
        throw error;
    }
}

/**
 * Refresh authentication token
 */
async function refreshAuthToken(refreshToken) {
    try {
        const response = await fetch(REFRESH_ENDPOINT, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ refreshToken })
        });
        
        if (!response.ok) {
            throw new Error('Token refresh failed');
        }
        
        const data = await response.json();
        
        // Update stored tokens
        localStorage.setItem(TOKEN_KEY, data.token);
        localStorage.setItem(REFRESH_TOKEN_KEY, data.refreshToken);
        
        // Extraer fecha de expiración desde el token JWT
        const tokenParts = data.token.split('.');
        if (tokenParts.length === 3) {
            try {
                const payload = JSON.parse(atob(tokenParts[1]));
                if (payload.exp) {
                    // payload.exp está en segundos desde epoch
                    const expiryDate = new Date(payload.exp * 1000);
                    localStorage.setItem(TOKEN_EXPIRY_KEY, expiryDate.toISOString());
                } else {
                    // Si no hay exp, establecer un valor predeterminado (1 hora)
                    const expiryTime = new Date();
                    expiryTime.setHours(expiryTime.getHours() + 1);
                    localStorage.setItem(TOKEN_EXPIRY_KEY, expiryTime.toISOString());
                }
            } catch (e) {
                console.error('Error parsing JWT token:', e);
                // Valor predeterminado en caso de error
                const expiryTime = new Date();
                expiryTime.setHours(expiryTime.getHours() + 1);
                localStorage.setItem(TOKEN_EXPIRY_KEY, expiryTime.toISOString());
            }
        } else {
            // Token mal formado, establecer tiempo predeterminado
            const expiryTime = new Date();
            expiryTime.setHours(expiryTime.getHours() + 1);
            localStorage.setItem(TOKEN_EXPIRY_KEY, expiryTime.toISOString());
        }
        
        return data;
    } catch (error) {
        console.error('Token refresh error:', error);
        // Clear stored auth data on refresh failure
        localStorage.removeItem(TOKEN_KEY);
        localStorage.removeItem(REFRESH_TOKEN_KEY);
        localStorage.removeItem(TOKEN_EXPIRY_KEY);
        localStorage.removeItem(USER_DATA_KEY);
        throw error;
    }
}

/**
 * Check if this is the first run (no admin exists)
 */
async function checkFirstRun() {
    try {
        // This is a simple check - in a real app, you'd create a specific endpoint
        // to check if admin setup is needed
        const response = await fetch(LOGIN_ENDPOINT, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username: 'admin', password: 'invalid-password-just-checking' })
        });
        
        // If we get a 404, assume the auth system or admin doesn't exist yet
        if (response.status === 404) {
            return true;
        }
        
        // If we get a 401, the auth system exists but credentials are wrong
        if (response.status === 401) {
            return false;
        }
        
        // Default to showing admin setup if we can't determine
        return false;
    } catch (error) {
        console.error('Error checking first run:', error);
        // If there's an error, show the admin setup to be safe
        return true;
    }
}

/**
 * Redirect to main application
 */
function redirectToMainApp() {
    // Previene el bucle infinito: si ya estamos en la página principal, no redirigir
    if (window.location.pathname === '/' || window.location.pathname === '') {
        console.log("Already on main page, not redirecting");
        return;
    }
    
    // Marcar que estamos redirigiendo para evitar múltiples redirecciones
    if (window.isRedirecting) {
        console.log("Redirect already in progress, ignoring duplicate request");
        return;
    }
    
    window.isRedirecting = true;
    
    // Uso de location.replace para evitar problemas con el historial del navegador
    console.log("Redirecting to main app...");
    setTimeout(() => {
        window.location.replace('/');
    }, 100);
}

/**
 * Logout - clear tokens and redirect to login
 */
function logout() {
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem(REFRESH_TOKEN_KEY);
    localStorage.removeItem(TOKEN_EXPIRY_KEY);
    localStorage.removeItem(USER_DATA_KEY);
    // Redirección a login con URL absoluta y sin parámetros
    const baseUrl = window.location.origin;
    window.location.replace(baseUrl + '/login');
}

// Create global AuthModule for use by other modules
const AuthModule = {
    isAuthenticated() {
        const token = localStorage.getItem(TOKEN_KEY);
        const tokenExpiry = localStorage.getItem(TOKEN_EXPIRY_KEY);
        return token && tokenExpiry && new Date(tokenExpiry) > new Date();
    },
    
    getToken() {
        return localStorage.getItem(TOKEN_KEY);
    },
    
    getUserData() {
        return JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
    },
    
    logout
};

// Expose the auth module globally
window.AuthModule = AuthModule;