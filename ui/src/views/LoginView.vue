<template>
  <Message severity="error" v-if="error">{{ error }}</Message>
  <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
    <div class="w-full md:mt-0 sm:max-w-md xl:p-0">
      <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
        <h1
          class="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white"
        >
          {{ $t('login.header') }}
        </h1>
        <form @submit.prevent="login" class="space-y-4 md:space-y-6">
          <div>
            <label
              for="email"
              class="block mb-2 text-md font-medium text-gray-900 dark:text-white"
              >{{ $t('login.email') }}</label
            >
            <InputText
              v-model="email"
              v-bind="emailAttr"
              inputId="email"
              :class="['w-full', { 'border-red-500': errors.email }]"
            />
            <span class="text-red-500 dark:text-red-800">{{ errors.email }}</span>
          </div>

          <div class="mb-4 flex flex-col">
            <label
              for="password"
              class="block mb-2 text-md font-medium text-gray-900 dark:text-white"
              >{{ $t('login.password') }}</label
            >
            <Password
              v-model="password"
              v-bind="passwordAttr"
              toggleMask
              inputId="password"
              :feedback="false"
              :inputClass="['w-full', { 'border-red-500': errors.password }]"
            />
            <span class="text-red-500 dark:text-red-800">{{ errors.password }}</span>
          </div>

          <!-- <div class="flex items-center justify-between"> FIXME: Do we need pwd reset?
            <a
              href="#"
              class="text-sm font-medium text-primary-600 hover:underline dark:text-primary-500"
              >{{ $t('login.forgotPassword') }}</a
            >
          </div> -->

          <Button type="submit" class="w-full justify-center">{{ $t('login.submit') }}</Button>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { authStore } from '../store/auth'
import router from '@/router'
import { useForm } from 'vee-validate'
import * as yup from 'yup'
import Message from 'primevue/message'
import Password from 'primevue/password'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import { useI18n } from 'vue-i18n'

const auth = authStore()
const error = ref('')

const { t } = useI18n()

const schema = yup.object().shape({
  email: yup.string().email(t('login.emailInvalid')).required(t('login.emailRequired')),
  password: yup.string().required(t('login.pwdRequired'))
})

const { defineField, resetForm, errors, handleSubmit } = useForm({
  validationSchema: schema
})

const [email, emailAttr] = defineField('email', { validateOnModelUpdate: false })
const [password, passwordAttr] = defineField('password')

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
    error.value = e.message || t('loginError')
  }
})
</script>
