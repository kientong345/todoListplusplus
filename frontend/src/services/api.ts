import api from "@/lib/api";
import type { LoginSchema, RegisterSchema } from "@/types/auth";
import type { CategoryCreateDto, CategoryDetailDto, CategoryMinimalDto, CategorySearchDto, CategoryUpdateDto } from "@/types/category";
import type { PageDto } from "@/types/page";
import type { TaskCreateDto, TaskDetailDto, TaskMinimalDto, TaskSearchDto, TaskUpdateDto } from "@/types/task";
import type { UserDto, UserUpdateDto } from "@/types/user";

// --- Auth ---
export const auth = {
  login: async (params: LoginSchema): Promise<{ access_token: string }> => {
    const response = await api.post('/auth/login', params);
    return response.data;
  },
  register: async (params: RegisterSchema) => {
    return api.post('/auth/register', params);
  },
  googleLogin: async (code: string): Promise<{ access_token: string }> => {
    console.log("code: ", code);
    const response = await api.post('/auth/google-login?code=' + code);
    return response.data;
  }
};

// --- User ---
export const user = {
  getMe: async (): Promise<UserDto> => {
    const response = await api.get('/users/me');
    return response.data;
  },
  updateMe: async (data: Partial<UserUpdateDto>) => {
    return api.patch('/users/me', data);
  }
};

// --- Categories ---
export const categories = {
  getAll: async (params: CategorySearchDto): Promise<PageDto<CategoryMinimalDto>> => {
    const response = await api.get('/categories', { params });
    return response.data;
  },
  getOne: async (id: string): Promise<CategoryDetailDto> => {
    const response = await api.get(`/categories/${id}`);
    return response.data;
  },
  create: async (params: CategoryCreateDto) => {
    return api.post('/categories', params);
  },
  update: async (id: string, params: CategoryUpdateDto) => {
    return api.patch(`/categories/${id}`, params);
  },
  delete: async (id: string) => {
    return api.delete(`/categories/${id}`);
  }
};

// --- Tasks ---
export const tasks = {
  getAll: async (categoryId: string, params: TaskSearchDto): Promise<PageDto<TaskMinimalDto>> => {
    const response = await api.get(`/categories/${categoryId}/tasks`, { params });
    return response.data;
  },
  getOne: async (categoryId: string, taskId: string): Promise<TaskDetailDto> => {
    const response = await api.get(`/categories/${categoryId}/tasks/${taskId}`);
    return response.data;
  },
  create: async (categoryId: string, params: TaskCreateDto) => {
    return api.post(`/categories/${categoryId}/tasks`, params);
  },
  update: async (categoryId: string, taskId: string, params: TaskUpdateDto) => {
    return api.patch(`/categories/${categoryId}/tasks/${taskId}`, params);
  },
  delete: async (categoryId: string, taskId: string) => {
    return api.delete(`/categories/${categoryId}/tasks/${taskId}`);
  }
};
