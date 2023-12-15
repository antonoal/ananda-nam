import { defineStore } from 'pinia'
import axiosClient from '@/axiosClient'
import router from '@/router'

export const authStore = defineStore('auth', {
  state: () => ({
    token: '',
    user: null as User | null
  }),
  persist: true,
  actions: {
    async login(email: string, password: string) {
      const response = await axiosClient.post<LoginResponse>('/login', {
        email: email,
        password: password
      })
      this.token = response.data.token
      this.user = response.data.user
    },
    logout() {
      this.token = ''
      this.user = null
      router.push('/login')
    },
    canSee(path) {
      const privs = privilegeMap.get(path)
      return privs === undefined || this.user?.privileges.some((v) => privs.includes(v.privilege))
    }
  }
})

enum Privilege {
  EDIT_STRUCTURE = 'EDIT_STRUCTURE',
  ASSIGN_STUDENTS = 'ASSIGN_STUDENTS',
  LOG_ATTENDANCE = 'LOG_ATTENDANCE',
  GENERATE_REPORTS = 'GENERATE_REPORTS',
  VIEW_STUDENT = 'VIEW_STUDENT'
}

const privilegeMap = new Map<string, Array<Privilege>>([['/persons', [Privilege.EDIT_STRUCTURE]]])

interface EntityPrivilege {
  entityId: number | null
  privilege: Privilege
}
interface User {
  id: number
  name: string
  email: string
  privileges: EntityPrivilege[]
}

interface LoginResponse {
  token: string
  user: User
}
