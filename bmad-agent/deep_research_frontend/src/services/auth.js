/**
 * Free Deep Research Frontend - Authentication Service
 * Comprehensive authentication and authorization system
 */

import apiClient from './api';

// Authentication state management
class AuthService {
  constructor() {
    this.token = localStorage.getItem('auth_token');
    this.user = JSON.parse(localStorage.getItem('user_data') || 'null');
    this.refreshToken = localStorage.getItem('refresh_token');
    this.listeners = new Set();
    
    // Set up token refresh timer
    this.setupTokenRefresh();
  }
  
  // Authentication methods
  async login(credentials) {
    try {
      const response = await apiClient.post('/auth/login', credentials);
      const { token, refreshToken, user, expiresIn } = response.data;
      
      this.setAuthData(token, refreshToken, user, expiresIn);
      this.notifyListeners('login', { user, token });
      
      return { success: true, user, token };
    } catch (error) {
      console.error('Login failed:', error);
      return { 
        success: false, 
        error: error.response?.data?.message || 'Login failed' 
      };
    }
  }
  
  async logout() {
    try {
      // Notify server about logout
      if (this.token) {
        await apiClient.post('/auth/logout', {}, {
          headers: { Authorization: `Bearer ${this.token}` }
        });
      }
    } catch (error) {
      console.warn('Logout request failed:', error);
    } finally {
      this.clearAuthData();
      this.notifyListeners('logout');
    }
  }
  
  async register(userData) {
    try {
      const response = await apiClient.post('/auth/register', userData);
      const { token, refreshToken, user, expiresIn } = response.data;
      
      this.setAuthData(token, refreshToken, user, expiresIn);
      this.notifyListeners('register', { user, token });
      
      return { success: true, user, token };
    } catch (error) {
      console.error('Registration failed:', error);
      return { 
        success: false, 
        error: error.response?.data?.message || 'Registration failed' 
      };
    }
  }
  
  async refreshAuthToken() {
    if (!this.refreshToken) {
      throw new Error('No refresh token available');
    }
    
    try {
      const response = await apiClient.post('/auth/refresh', {
        refreshToken: this.refreshToken
      });
      
      const { token, refreshToken, expiresIn } = response.data;
      this.setAuthData(token, refreshToken, this.user, expiresIn);
      
      return token;
    } catch (error) {
      console.error('Token refresh failed:', error);
      this.clearAuthData();
      this.notifyListeners('tokenExpired');
      throw error;
    }
  }
  
  async resetPassword(email) {
    try {
      const response = await apiClient.post('/auth/reset-password', { email });
      return { success: true, message: response.data.message };
    } catch (error) {
      console.error('Password reset failed:', error);
      return { 
        success: false, 
        error: error.response?.data?.message || 'Password reset failed' 
      };
    }
  }
  
  async confirmPasswordReset(token, newPassword) {
    try {
      const response = await apiClient.post('/auth/confirm-reset', {
        token,
        newPassword
      });
      return { success: true, message: response.data.message };
    } catch (error) {
      console.error('Password reset confirmation failed:', error);
      return { 
        success: false, 
        error: error.response?.data?.message || 'Password reset confirmation failed' 
      };
    }
  }
  
  async changePassword(currentPassword, newPassword) {
    try {
      const response = await apiClient.post('/auth/change-password', {
        currentPassword,
        newPassword
      });
      return { success: true, message: response.data.message };
    } catch (error) {
      console.error('Password change failed:', error);
      return { 
        success: false, 
        error: error.response?.data?.message || 'Password change failed' 
      };
    }
  }
  
  async updateProfile(profileData) {
    try {
      const response = await apiClient.put('/auth/profile', profileData);
      const updatedUser = response.data.user;
      
      this.user = updatedUser;
      localStorage.setItem('user_data', JSON.stringify(updatedUser));
      this.notifyListeners('profileUpdated', updatedUser);
      
      return { success: true, user: updatedUser };
    } catch (error) {
      console.error('Profile update failed:', error);
      return { 
        success: false, 
        error: error.response?.data?.message || 'Profile update failed' 
      };
    }
  }
  
  // Session management
  async validateSession() {
    if (!this.token) {
      return false;
    }
    
    try {
      const response = await apiClient.get('/auth/validate', {
        headers: { Authorization: `Bearer ${this.token}` }
      });
      
      if (response.data.valid) {
        // Update user data if provided
        if (response.data.user) {
          this.user = response.data.user;
          localStorage.setItem('user_data', JSON.stringify(this.user));
        }
        return true;
      } else {
        this.clearAuthData();
        return false;
      }
    } catch (error) {
      console.error('Session validation failed:', error);
      
      // Try to refresh token if validation fails
      if (error.response?.status === 401 && this.refreshToken) {
        try {
          await this.refreshAuthToken();
          return true;
        } catch (refreshError) {
          this.clearAuthData();
          return false;
        }
      }
      
      this.clearAuthData();
      return false;
    }
  }
  
  // Authorization methods
  hasPermission(permission) {
    if (!this.user || !this.user.permissions) {
      return false;
    }
    
    return this.user.permissions.includes(permission) || 
           this.user.permissions.includes('admin');
  }
  
  hasRole(role) {
    if (!this.user || !this.user.roles) {
      return false;
    }
    
    return this.user.roles.includes(role);
  }
  
  canAccessResource(resource, action = 'read') {
    if (!this.user) {
      return false;
    }
    
    // Admin can access everything
    if (this.hasRole('admin')) {
      return true;
    }
    
    // Check specific permissions
    const permission = `${resource}:${action}`;
    return this.hasPermission(permission);
  }
  
  // Utility methods
  isAuthenticated() {
    return !!this.token && !!this.user;
  }
  
  getUser() {
    return this.user;
  }
  
  getToken() {
    return this.token;
  }
  
  getUserId() {
    return this.user?.id;
  }
  
  getUserRole() {
    return this.user?.roles?.[0] || 'user';
  }
  
  // Private methods
  setAuthData(token, refreshToken, user, expiresIn) {
    this.token = token;
    this.refreshToken = refreshToken;
    this.user = user;
    
    // Store in localStorage
    localStorage.setItem('auth_token', token);
    localStorage.setItem('refresh_token', refreshToken);
    localStorage.setItem('user_data', JSON.stringify(user));
    
    if (expiresIn) {
      const expirationTime = Date.now() + (expiresIn * 1000);
      localStorage.setItem('token_expiration', expirationTime.toString());
    }
    
    // Update API client default headers
    apiClient.defaults.headers.common['Authorization'] = `Bearer ${token}`;
    
    // Setup token refresh
    this.setupTokenRefresh();
  }
  
  clearAuthData() {
    this.token = null;
    this.refreshToken = null;
    this.user = null;
    
    // Clear localStorage
    localStorage.removeItem('auth_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user_data');
    localStorage.removeItem('token_expiration');
    
    // Clear API client headers
    delete apiClient.defaults.headers.common['Authorization'];
    
    // Clear refresh timer
    if (this.refreshTimer) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = null;
    }
  }
  
  setupTokenRefresh() {
    if (this.refreshTimer) {
      clearTimeout(this.refreshTimer);
    }
    
    const expirationTime = localStorage.getItem('token_expiration');
    if (!expirationTime || !this.refreshToken) {
      return;
    }
    
    const timeUntilExpiration = parseInt(expirationTime) - Date.now();
    const refreshTime = Math.max(timeUntilExpiration - 300000, 60000); // Refresh 5 minutes before expiration, but at least in 1 minute
    
    if (refreshTime > 0) {
      this.refreshTimer = setTimeout(async () => {
        try {
          await this.refreshAuthToken();
        } catch (error) {
          console.error('Automatic token refresh failed:', error);
        }
      }, refreshTime);
    }
  }
  
  // Event listeners
  addListener(callback) {
    this.listeners.add(callback);
    return () => this.listeners.delete(callback);
  }
  
  removeListener(callback) {
    this.listeners.delete(callback);
  }
  
  notifyListeners(event, data) {
    this.listeners.forEach(callback => {
      try {
        callback(event, data);
      } catch (error) {
        console.error('Error in auth listener:', error);
      }
    });
  }
}

// Create singleton instance
const authService = new AuthService();

// React hook for authentication
export const useAuth = () => {
  const [authState, setAuthState] = React.useState({
    isAuthenticated: authService.isAuthenticated(),
    user: authService.getUser(),
    token: authService.getToken(),
    loading: false
  });
  
  React.useEffect(() => {
    const unsubscribe = authService.addListener((event, data) => {
      switch (event) {
        case 'login':
        case 'register':
          setAuthState({
            isAuthenticated: true,
            user: data.user,
            token: data.token,
            loading: false
          });
          break;
        case 'logout':
        case 'tokenExpired':
          setAuthState({
            isAuthenticated: false,
            user: null,
            token: null,
            loading: false
          });
          break;
        case 'profileUpdated':
          setAuthState(prev => ({
            ...prev,
            user: data
          }));
          break;
      }
    });
    
    // Validate session on mount
    authService.validateSession().then(isValid => {
      if (!isValid && authState.isAuthenticated) {
        setAuthState({
          isAuthenticated: false,
          user: null,
          token: null,
          loading: false
        });
      }
    });
    
    return unsubscribe;
  }, []);
  
  const login = async (credentials) => {
    setAuthState(prev => ({ ...prev, loading: true }));
    const result = await authService.login(credentials);
    setAuthState(prev => ({ ...prev, loading: false }));
    return result;
  };
  
  const logout = async () => {
    setAuthState(prev => ({ ...prev, loading: true }));
    await authService.logout();
    // State will be updated by the listener
  };
  
  const register = async (userData) => {
    setAuthState(prev => ({ ...prev, loading: true }));
    const result = await authService.register(userData);
    setAuthState(prev => ({ ...prev, loading: false }));
    return result;
  };
  
  return {
    ...authState,
    login,
    logout,
    register,
    resetPassword: authService.resetPassword.bind(authService),
    changePassword: authService.changePassword.bind(authService),
    updateProfile: authService.updateProfile.bind(authService),
    hasPermission: authService.hasPermission.bind(authService),
    hasRole: authService.hasRole.bind(authService),
    canAccessResource: authService.canAccessResource.bind(authService)
  };
};

// Protected route component
export const ProtectedRoute = ({ children, permission, role, fallback }) => {
  const { isAuthenticated, user } = useAuth();
  
  if (!isAuthenticated) {
    return fallback || <Navigate to="/login" replace />;
  }
  
  if (permission && !authService.hasPermission(permission)) {
    return fallback || <Navigate to="/unauthorized" replace />;
  }
  
  if (role && !authService.hasRole(role)) {
    return fallback || <Navigate to="/unauthorized" replace />;
  }
  
  return children;
};

export default authService;
