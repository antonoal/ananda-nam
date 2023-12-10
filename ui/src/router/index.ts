import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import LayoutView from '../views/LayoutView.vue'
import PersonsView from '../views/PersonsView.vue'
import LoginView from '@/views/LoginView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: LayoutView,
      meta: { requiresAuth: true },
      children: [
        {path: '', component: HomeView},
        {path: '/about', component: AboutView},
        {path: '/persons', component: PersonsView}
      ]
    },
    {
      path: '/login',
      component: LoginView
    }
    // {
    //   path: '/about',
    //   name: 'about',
    //   // route level code-splitting
    //   // this generates a separate chunk (About.[hash].js) for this route
    //   // which is lazy-loaded when the route is visited.
    //   component: () => import('../views/AboutView.vue')
    // }
  ]
})

export default router
