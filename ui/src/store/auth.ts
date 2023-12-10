import { defineStore } from 'pinia';
import axiosClient from '@/axiosClient';

export const authStore = defineStore('auth', {
  state: () => ({
    token: 'xx',
    user: {name: 'Bob Synclair', email: 'a@a.a'} as User | null //null as User | null,
  }),
  persist: {
    storage: sessionStorage,
  },
  actions: {
    async login(username: string, password: string) {
      const response = await axiosClient.post<LoginResponse>('/login', {'username': username, 'password': password});
      this.token = response.data.token;
      const userResponse = await axiosClient.get<MeResponse>('/me');
      this.user = userResponse.data.user;
    },
    logout() {
      this.token = '';
      this.user = null;
    },
  },
});

interface User {
  name: string,
  email: string,
}

interface MeResponse {
  user: User
}

interface LoginResponse {
  token: string;
}