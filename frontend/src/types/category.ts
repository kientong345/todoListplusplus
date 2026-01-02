export interface CategoryMinimalDto {
  id: string;
  userId: string;
  name: string;
  imageUrl?: string | null;
  description?: string | null;
  taskCount: number;
}

export interface CategoryDetailDto {
  id: string;
  userId: string;
  name: string;
  imageUrl?: string | null;
  description?: string | null;
  createdAt?: string | null;
  updatedAt?: string | null;
  taskCount: number;
  openedTaskCount: number;
  canceledTaskCount: number;
  doneTaskCount: number;
  progress: number;
}

export interface CategoryCreateDto {
  name: string;
  imageUrl?: string | null;
  description?: string | null;
}

export interface CategoryUpdateDto {
  name?: string | null;
  imageUrl?: string | null;
  description?: string | null;
}

export interface CategorySearchDto {
  namePattern?: string | null;
  page: number;
  pageSize: number;
  sortBy: string;
}
