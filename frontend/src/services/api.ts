import api from "@/lib/api";
import type { CategoryMinimal, Category, Task, User } from "@/types";

export interface PaginatedResponse<T> {
  items: T[];
  totalItems: number;
  totalPages: number;
  page: number;
  pageSize: number;
}

// --- Auth ---
export const auth = {
  login: async (email: string, password: string): Promise<{ access_token: string }> => {
    const response = await api.post('/auth/login', { email, password });
    return response.data;
  },
  register: async (displayName: string, email: string, password: string) => {
    return api.post('/auth/register', { displayName, email, password });
  },
  googleLogin: async (code: string): Promise<{ access_token: string }> => {
    console.log("code: ", code);
    const response = await api.post('/auth/google-login?code=' + code);
    return response.data;
  }
};

// --- User ---
export const user = {
  getMe: async (): Promise<User> => {
    const response = await api.get('/users/me');
    return response.data;
  },
  updateMe: async (data: Partial<User>) => {
    return api.patch('/users/me', data);
  }
};

// --- Categories ---
export const categories = {
  getAll: async (page = 1, pageSize = 20, namePattern = '', sortBy = 'new-update'): Promise<PaginatedResponse<CategoryMinimal>> => {
    const params = { page, pageSize, namePattern, sortBy };
    const response = await api.get('/categories', { params });
    return response.data;
  },
  getOne: async (id: string): Promise<Category> => {
    const response = await api.get(`/categories/${id}`);
    return response.data;
  },
  create: async (data: { name: string; imageUrl?: string | null; description?: string | null }) => {
    return api.post('/categories', data);
  },
  update: async (id: string, data: { name?: string; imageUrl?: string | null; description?: string | null }) => {
    return api.patch(`/categories/${id}`, data);
  },
  delete: async (id: string) => {
    return api.delete(`/categories/${id}`);
  }
};

// --- Tasks ---
export const tasks = {
  getAll: async (categoryId: string, page = 1, pageSize = 20, status?: string[], titlePattern = '', sortBy = 'latest'): Promise<PaginatedResponse<Task>> => {
    const params: any = { page, pageSize, sortBy, titlePattern };
    if (status && status.length > 0) {
      params.status = status; 
    }
    const response = await api.get(`/categories/${categoryId}/tasks`, { params });
    return response.data;
  },
  getOne: async (categoryId: string, taskId: string): Promise<Task> => {
    const response = await api.get(`/categories/${categoryId}/tasks/${taskId}`);
    return response.data;
  },
  create: async (categoryId: string, data: Partial<Task>) => {
    // Ensure defaults or required fields
    return api.post(`/categories/${categoryId}/tasks`, data);
  },
  update: async (categoryId: string, taskId: string, data: Partial<Task>) => {
    return api.patch(`/categories/${categoryId}/tasks/${taskId}`, data);
  },
  delete: async (categoryId: string, taskId: string) => {
    return api.delete(`/categories/${categoryId}/tasks/${taskId}`);
  }
};
