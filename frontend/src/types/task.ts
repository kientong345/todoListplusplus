export interface TaskMinimal {
  id: number;
  title: string;
  status: string;
  expiresAt?: string;
  cycleTime?: string;
}

export interface TaskDetail extends TaskMinimal {
  categoryId: number;
  categoryName: string;
  description?: string;
  userComment?: string;
  createdAt?: string;
  updatedAt?: string;
  preNotifyTime?: string;
}

export interface TaskCreateDto {
  title: string;
  description?: string;
  status: string;
  userComment?: string;
  expiresAt?: string;
  cycleTime?: string;
  preNotifyTime?: string;
}

export interface TaskUpdateDto {
  title?: string;
  description?: string;
  status?: string;
  userComment?: string;
}

export interface TaskSearchDto {
  titlePattern?: string;
  status?: string[];
  page: number;
  pageSize: number;
  sortBy?: string;
}
