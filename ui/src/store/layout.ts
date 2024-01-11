import { defineStore } from 'pinia'
import type School from '@/models/School'

export const layoutStore = defineStore('layout', {
  state: () => ({
    school: null as School | null
  }),
  persist: true,
  actions: {
    setSchool(school: School) {
      this.school = school
    }
  }
})
