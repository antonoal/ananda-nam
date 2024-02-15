import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import PrimeVue from 'primevue/config'
import 'primeicons/primeicons.css'
import App from './App.vue'
import router from './router'
import Tailwind from 'primevue/passthrough/tailwind'
import { useAuthStore } from './store/auth'
import { useTokenStore } from './store/token'
import i18n from './i18n'

const app = createApp(App)
const pinia = createPinia()

pinia.use(piniaPluginPersistedstate)

app.use(i18n)
app.use(pinia)
app.use(router)
app.use(PrimeVue, { unstyled: true, pt: Tailwind })

const authStore = useAuthStore()
const tokenStore = useTokenStore()

router.beforeEach((to, from, next) => {
  if (to.meta.requiresAuth && !tokenStore.getToken()) {
    localStorage.setItem('intendedRoute', to.fullPath)
    next('/login')
  } else if (!authStore.canSee(to.path)) {
    next('/forbidden')
  } else {
    next()
  }
})

app.mount('#app')
