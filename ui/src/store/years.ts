import { defineStore } from 'pinia'
import axiosClient from '@/axiosClient'
import type Year from '@/models/Year'

export const yearsStore = defineStore('years', {
  state: () => ({
    years: [] as Year[]
  }),
  actions: {
    async fetchYears(schoolId: number) {
      try {
        const response = await axiosClient.get<YearsResponse>(`/schools/${schoolId}/years`)
        this.years = response.data.years
      } catch (error) {
        console.error('Error fetching years:', error)
        throw error
      }
    },
    async addYear(schoolId: number, newYear: NewYear) {
      try {
        await axiosClient.post(`/schools/${schoolId}/years`, { year: newYear })
      } catch (error) {
        console.error('Error adding year:', error)
        throw error
      }
    },
    async deleteYear(schoolId: number, id: number) {
      try {
        await axiosClient.delete(`/schools/${schoolId}/years/${id}`)
      } catch (error) {
        console.error('Error deleting year:', error)
        throw error
      }
    },
    async updateYear(schoolId: number, id: number, newYear: NewYear) {
      try {
        await axiosClient.put(`/schools/${schoolId}/years/${id}`, { year: newYear })
      } catch (error) {
        console.error('Error updating year:', error)
        throw error
      }
    }
  }
})

interface YearsResponse {
  years: Array<Year>
}

export interface NewYear extends Omit<Year, 'id'> {}
