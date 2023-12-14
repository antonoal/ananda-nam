import { defineStore } from 'pinia'
import axiosClient from '@/axiosClient'
import type Stream from '@/models/Stream'

export const streamsStore = defineStore('streams', {
  state: () => ({
    streams: [] as Stream[]
  }),
  actions: {
    async fetchStreams() {
      try {
        const response = await axiosClient.get<StreamsResponse>('/streams')
        this.streams = response.data.streams
      } catch (error) {
        console.error('Error fetching persons:', error)
        throw error
      }
    },
    async addStream(newStream: NewStream) {
      try {
        await axiosClient.post('/streams', { stream: newStream })
      } catch (error) {
        console.error('Error adding stream:', error)
        throw error
      }
    },
    async deleteStream(id: number) {
      try {
        await axiosClient.delete(`/streams/${id}`)
      } catch (error) {
        console.error('Error deleting stream:', error)
        throw error
      }
    },
    async updateStream(id: number, newStream: NewStream) {
      try {
        await axiosClient.put(`/streams/${id}`, { stream: newStream })
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
