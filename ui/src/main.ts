import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import PrimeVue from 'primevue/config'
import 'primeicons/primeicons.css'
import App from './App.vue'
import router from './router'
import Tailwind from 'primevue/passthrough/tailwind'
import { authStore } from './store/auth'
import i18n from './i18n'

const app = createApp(App)
const pinia = createPinia()

pinia.use(piniaPluginPersistedstate)

app.use(i18n)
app.use(pinia)
app.use(router)
app.use(PrimeVue, { unstyled: true, pt: Tailwind })

const auth = authStore()

router.beforeEach((to, from, next) => {
  if (to.meta.requiresAuth && !auth.token) {
    localStorage.setItem('intendedRoute', to.fullPath)
    next('/login')
  } else if (!auth.canSee(to.path)) {
    next('/forbidden')
  } else {
    next()
  }
})

app.mount('#app')
