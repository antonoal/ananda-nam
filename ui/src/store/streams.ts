import { defineStore } from 'pinia'
import axiosClient from '@/axiosClient'
import type Stream from '@/models/Stream'

export const useStreamsStore = defineStore('streams', {
  state: () => ({
    streams: [] as Stream[]
  }),
  actions: {
    async fetchStreams(schoolId: number) {
      try {
        const response = await axiosClient.get<StreamsResponse>(`/schools/${schoolId}/streams`)
        this.streams = response.data.streams
      } catch (error) {
        console.error('Error fetching streams:', error)
        throw error
      }
    },
    async addStream(schoolId: number, newStream: NewStream) {
      try {
        await axiosClient.post(`/schools/${schoolId}/streams`, { stream: newStream })
      } catch (error) {
        console.error('Error adding stream:', error)
        throw error
      }
    },
    async deleteStream(schoolId: number, id: number) {
      try {
        await axiosClient.delete(`/schools/${schoolId}/streams/${id}`)
      } catch (error) {
        console.error('Error deleting stream:', error)
        throw error
      }
    },
    async updateStream(schoolId: number, id: number, newStream: NewStream) {
      try {
        await axiosClient.put(`/schools/${schoolId}/streams/${id}`, { stream: newStream })
      } catch (error) {
        console.error('Error updating stream:', error)
        throw error
      }
    }
  }
})

interface StreamsResponse {
  streams: Array<Stream>
}

export interface NewStream extends Omit<Stream, 'id'> {}
