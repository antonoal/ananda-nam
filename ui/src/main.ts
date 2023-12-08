import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import App from './App.vue'
import router from './router'
import Tailwind from "primevue/passthrough/tailwind";


const app = createApp(App)

app.use(router)
app.use(createPinia())
app.use(PrimeVue, { ripple: true, unstyled: true, pt: Tailwind });

app.mount('#app')
