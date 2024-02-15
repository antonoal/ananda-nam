import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import StreamsView from '../views/StreamsView.vue'
import LayoutView from '../views/LayoutView.vue'
import PersonsView from '../views/PersonsView.vue'
import LoginView from '@/views/LoginView.vue'
import ForbiddenView from '@/views/ForbiddenView.vue'
import SchoolsView from '@/views/SchoolsView.vue'
import YearsView from '@/views/YearsView.vue'
import GroupsView from '@/views/GroupsView.vue'

export {}

// Custom routes metadata type
declare module 'vue-router' {
  interface RouteMeta {
    breadcrumbsLevel?: number
  }
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: LayoutView,
      meta: { requiresAuth: true },
      children: [
        { path: '', component: HomeView },
        { path: '/schools', component: SchoolsView },
        {
          name: 'years-view',
          path: '/schools/:schoolId/years',
          component: YearsView,
          meta: { breadcrumbsLevel: 1 }
        },
        {
          name: 'streams-view',
          path: '/schools/:schoolId/streams',
          component: StreamsView,
          meta: { breadcrumbsLevel: 1 }
        },
        {
          name: 'groups-view',
          path: '/schools/:schoolId/streams/:streamId/groups',
          component: GroupsView,
          meta: { breadcrumbsLevel: 2 }
        },
        { path: '/persons', component: PersonsView }
      ]
    },
    {
      path: '/login',
      component: LoginView
    },
    {
      path: '/forbidden',
      component: ForbiddenView
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
