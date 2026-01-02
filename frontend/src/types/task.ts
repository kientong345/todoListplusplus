export interface TaskMinimalDto {
  id: string;
  title: string;
  status: string;
  expiresAt?: string | null;
  cycleTime?: string | null;
}

export interface TaskDetailDto {
  id: string;
  categoryId: string;
  categoryName: string;
  title: string;
  description?: string | null;
  status: string;
  userComment?: string | null;
  createdAt?: string | null;
  updatedAt?: string | null;
  expiresAt?: string | null;
  cycleTime?: string | null;
  notifyTime?: string | null;
}

export interface TaskCreateDto {
  title: string;
  description?: string | null;
  expiresAt?: string | null;
  cycleTime?: string | null;
  notifyTime?: string | null;
}

export interface TaskUpdateDto {
  title?: string | null;
  description?: string | null;
  status?: string | null;
  userComment?: string | null;
  expiresAt?: string | null;
  cycleTime?: string | null;
  notifyTime?: string | null;
}

export interface TaskSearchDto {
  titlePattern?: string | null;
  status?: string[] | null;
  page: number;
  pageSize: number;
  sortBy: string;
}