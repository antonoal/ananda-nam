<template>
  <div class="container my-8">
    <h1 class="text-2xl font-bold mb-4">People List</h1>

    <form @submit.prevent="onSubmit">
      <div class="flex gap-6">
        <div class="mb-4 flex flex-col">
          <label for="newName" class="block text-sm font-medium text-gray-700">Name:</label>
          <input
            v-model="name"
            v-bind="nameAttrs"
            name="name"
            type="text"
            class="mt-1 p-2 border rounded-md"
            :class="{ 'border-red-500': errors.name }"
          />
          <span class="text-red-500">{{ errors.name }}</span>
        </div>

        <div class="mb-4 flex flex-col">
          <label for="newEmail" class="block text-sm font-medium text-gray-700">Email:</label>
          <input
            v-model="email"
            v-bind="emailAttrs"
            name="email"
            type="email"
            class="mt-1 p-2 border rounded-md"
            :class="{ 'border-red-500': errors.email }"
          />
          <span class="text-red-500">{{ errors.email }}</span>
        </div>
      </div>
      <button class="mt-4 p-2 bg-blue-500 text-white rounded-md">Submit</button>
    </form>

    <DataTable
      stripedRows
      showGridlines
      dataKey="id"
      :globalFilterFields="['name']"
      class="w-full border-t border-b border-gray-200 divide-y divide-gray-200"
      :value="persons"
    >
      <Column field="id" header="ID" class="text-center"></Column>
      <Column field="name" header="Name"></Column>
      <Column field="email" header="Email"></Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { personsStore } from '@/store/persons'
import type Person from '@/models/Person'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { useForm } from 'vee-validate'
import * as yup from 'yup'

const store = personsStore()
const persons = ref<Person[]>([])

const schema = yup.object().shape({
  email: yup.string().email('Email is invalid').required('Email is required'),
  name: yup
    .string()
    .min(3, 'Name must be at least 3 characters long')
    .test('empty-check', "Name can't be empty", (name) => name?.trim.length == 0)
    .required('First Name is required')
  // password: yup
  //   .string()
  //   .min(6, 'Password must be at least 6 characters')
  //   .required('Password is required'),
  // confirmPassword: yup
  //   .string()
  //   .oneOf([yup.ref('password')], 'Passwords must match')
  //   .required('Confirm Password is required')
})

const { defineField, resetForm, errors, handleSubmit } = useForm({
  validationSchema: schema
})

const [email, emailAttrs] = defineField('email')
const [name, nameAttrs] = defineField('name')

onMounted(async () => {
  await store.fetchPersons()
  persons.value = store.persons
})

const onSubmit = handleSubmit(async () => {
  const personToAdd = {
    name: name.value,
    email: email.value,
    password: 'newPerson.value.password'
  }

  await store.addPerson(personToAdd)
  await store.fetchPersons()

  console.log('Successfully submitted!')

  persons.value = store.persons
  resetForm()
})
</script>

<style scoped lang="css">
/* Striping rows */
.p-datatable-row:nth-child(even) {
  background-color: #f3f4f6; /* Customize the background color as needed */
}
</style>
