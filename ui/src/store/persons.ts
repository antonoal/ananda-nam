import type Person from '@/models/Person';
import { defineStore } from 'pinia';
import axiosClient from '@/axiosClient';

export const personsStore = defineStore('persons', {
  state: () => ({
    persons: [] as Person[],
  }),
  actions: {
    async fetchPersons() {
      try {
        // const response = await axiosClient.get<PersonsResponse>('/persons');

        // this.persons = response.data.persons;
      } catch (error) {
        console.error('Error fetching persons:', error);
      }
    },
    async addPerson(newPerson: NewPerson) {
      try {
        // const response = await axiosClient.post('/persons', { person: newPerson});
        
        // console.log('Person added successfully:', response.data);
      } catch (error) {
        console.error('Error adding person:', error);
        throw error;
      }
    },
  },
});

interface PersonsResponse {
  persons: Array<Person>;
}

export interface NewPerson extends Omit<Person, 'id'> {
  password: string;
}