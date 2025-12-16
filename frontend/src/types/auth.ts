export interface User {
  id: number;
  displayName: string;
  email: string;
  avatarUrl?: string;
  description?: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface LoginResponse {
  access_token: string;
}

export interface RegisterResponse {
  // Status 201
}
