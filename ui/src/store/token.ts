import { defineStore } from 'pinia'

export const useTokenStore = defineStore('token', {
  state: () => ({
    token: ''
  }),
  persist: true,
  actions: {
    setToken(token: string) {
      this.token = token
    },
    getToken() {
      return this.token
    }
  }
})
