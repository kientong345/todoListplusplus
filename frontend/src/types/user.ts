export interface UserDto {
  id: string;
  displayName: string;
  email: string;
  avatarUrl?: string | null;
  description?: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface UserUpdateDto {
  displayName?: string | null;
  avatarUrl?: string | null;
  description?: string | null;
}