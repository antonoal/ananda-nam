import { defineStore } from 'pinia'
import axiosClient from '@/axiosClient'
import type School from '@/models/School'

export const useSchoolsStore = defineStore('schools', {
  state: () => ({
    schools: [] as School[]
  }),
  actions: {
    async fetchSchools() {
      try {
        const response = await axiosClient.get<SchoolsResponse>('/schools')
        this.schools = response.data.schools
      } catch (error) {
        console.error('Error fetching schools:', error)
        throw error
      }
    },
    async addSchool(newSchool: NewSchool) {
      try {
        await axiosClient.post('/schools', { school: newSchool })
      } catch (error) {
        console.error('Error adding school:', error)
        throw error
      }
    },
    async deleteSchool(id: number) {
      try {
        await axiosClient.delete(`/schools/${id}`)
      } catch (error) {
        console.error('Error deleting school:', error)
        throw error
      }
    },
    async updateSchool(id: number, newSchool: NewSchool) {
      try {
        await axiosClient.put(`/schools/${id}`, { school: newSchool })
      } catch (error) {
        console.error('Error updating school:', error)
        throw error
      }
    }
  }
})

interface SchoolsResponse {
  schools: Array<School>
}

export interface NewSchool extends Omit<School, 'id'> {}
