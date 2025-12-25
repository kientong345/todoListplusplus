import { createContext, useContext, useState, useEffect, type ReactNode } from 'react';
import { user as userService } from "@/services/api";
import type { User } from "@/types";

interface AuthContextType {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: User | null;
  login: (token: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  const fetchUser = async () => {
    try {
      const userData = await userService.getMe();
      setUser(userData);
      setIsAuthenticated(true);
    } catch (error) {
      console.error("Failed to fetch user", error);
      logout();
    }
  };

  useEffect(() => {
    // Check for token on mount
    const token = localStorage.getItem('token');
    if (token) {
      fetchUser().finally(() => setIsLoading(false));
    } else {
      setIsLoading(false);
    }
  }, []);

  const login = async (token: string) => {
    localStorage.setItem('token', token);
    await fetchUser();
  };

  const logout = () => {
    localStorage.removeItem('token');
    setIsAuthenticated(false);
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ isAuthenticated, isLoading, user, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
