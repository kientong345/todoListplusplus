export interface CategoryMinimal {
  id: string;
  userId: string;
  name: string;
  imageUrl?: string | null;
  description?: string | null;
  taskCount: number;
}

export interface Category extends CategoryMinimal {
  createdAt: string;
  updatedAt: string;
  openedTaskCount: number;
  canceledTaskCount: number;
  doneTaskCount: number;
  progress: number;
}


export interface Task {
  id: string;
  categoryId: string;
  userId: string;
  title: string;
  description?: string | null;
  status: 'open' | 'done' | 'canceled';
  expiresAt?: string | null;
  cycleTime?: 'none' | 'daily' | 'weekly' | 'monthly' | null;
  createdAt: string;
  updatedAt: string;
}

export interface User {
  id: string;
  displayName: string;
  email: string;
  avatarUrl?: string | null;
  description?: string | null;
  createdAt: string;
  updatedAt: string;
}
