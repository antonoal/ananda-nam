import { defineStore } from 'pinia'
import type School from '@/models/School'
import type Stream from '@/models/Stream'

export const useLayoutStore = defineStore('layout', {
  state: () => ({
    school: null as School | null,
    stream: null as Stream | null
  }),
  persist: true,
  actions: {
    setSchool(school: School) {
      this.school = school
    },
    resetSchool() {
      this.school = null
    },
    setStream(stream: Stream) {
      this.stream = stream
    },
    resetStream() {
      this.stream = null
    }
  }
})
