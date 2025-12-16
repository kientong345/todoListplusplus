export interface CategoryMinimal {
  id: number;
  userId: number;
  name: string;
  imageUrl?: string;
  description?: string;
  taskCount: number;
}

export interface CategoryDetail extends CategoryMinimal {
  createdAt?: string;
  updatedAt?: string;
  openedTaskCount: number;
  canceledTaskCount: number;
  doneTaskCount: number;
  progress: number;
}

export interface CategoryCreateDto {
  name: string;
  imageUrl?: string;
  description?: string;
}

export interface CategoryUpdateDto {
  name?: string;
  imageUrl?: string;
  description?: string;
}

export interface CategorySearchDto {
  namePattern?: string;
  page: number;
  pageSize: number;
  sortBy?: string;
}


