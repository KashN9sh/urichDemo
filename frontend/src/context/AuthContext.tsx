import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from 'react';
import { useNavigate } from 'react-router-dom';
import { clearToken, getToken, login as apiLogin, setToken } from '../api/client';

interface AuthContextValue {
  token: string | null;
  isAuthenticated: boolean;
  login: (username: string, password: string) => Promise<{ ok: boolean; error?: string }>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setTokenState] = useState<string | null>(() => getToken());
  const navigate = useNavigate();

  useEffect(() => {
    const on401 = () => {
      setTokenState(null);
      navigate('/login', { replace: true });
    };
    window.addEventListener('auth:401', on401);
    return () => window.removeEventListener('auth:401', on401);
  }, [navigate]);

  const login = useCallback(
    async (username: string, password: string) => {
      try {
        const res = await apiLogin(username, password);
        if (res.token) {
          setToken(res.token);
          setTokenState(res.token);
          return { ok: true };
        }
        return { ok: false, error: (res as { detail?: string }).detail ?? 'Login failed' };
      } catch (e) {
        return {
          ok: false,
          error: e instanceof Error ? e.message : 'Login failed',
        };
      }
    },
    []
  );

  const logout = useCallback(() => {
    clearToken();
    setTokenState(null);
    navigate('/login', { replace: true });
  }, [navigate]);

  return (
    <AuthContext.Provider
      value={{
        token,
        isAuthenticated: !!token,
        login,
        logout,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within AuthProvider');
  return ctx;
}
