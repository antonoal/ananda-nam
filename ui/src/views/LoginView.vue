<template>
  <Message severity="error" v-if="error">{{ error }}</Message>
  <form @submit.prevent="login">
    <div class="mb-4 flex flex-col">
      <label for="email">Username</label>
      <input
        v-model="email"
        v-bind="emailAttrs"
        type="text"
        id="email"
        class="mt-1 p-2 border rounded-md"
        :class="{ 'border-red-500': errors.email }"
      />
      <span class="text-red-500">{{ errors.email }}</span>
    </div>

    <div class="mb-4 flex flex-col">
      <label for="password">Password</label>
      <input
        v-model="password"
        v-bind="passwordAttrs"
        type="password"
        id="password"
        class="mt-1 p-2 border rounded-md"
        :class="{ 'border-red-500': errors.password }"
      />
      <span class="text-red-500">{{ errors.password }}</span>
    </div>

    <button class="mt-4 p-2 bg-blue-500 text-white rounded-md">Submit</button>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { authStore } from '../store/auth'
import router from '@/router'
import { useForm } from 'vee-validate'
import * as yup from 'yup'
import Message from 'primevue/message'

const auth = authStore()
const error = ref('')

const schema = yup.object().shape({
  email: yup.string().email('Email is invalid').required('Email is required'),
  password: yup.string().required('Password is required')
})

const { defineField, resetForm, errors, handleSubmit } = useForm({
  validationSchema: schema
})

const [email, emailAttrs] = defineField('email')
const [password, passwordAttrs] = defineField('password')

const login = handleSubmit(async () => {
  error.value = ''
  try {
    await auth.login(email.value, password.value)
    const intendedRoute = localStorage.getItem('intendedRoute')
    if (intendedRoute) {
      localStorage.removeItem('intendedRoute')
      router.push(intendedRoute)
    } else {
      router.push('/')
    }
    resetForm()
  } catch (e: any) {
    error.value = e.message || 'An error occurred during login.'
  }
})
</script>
