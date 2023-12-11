<template>
  <div class="relative z-0 flex h-full w-full overflow-hidden">
    <div
      class="dark flex-shrink-0 overflow-x-hidden bg-black"
      :class="[sidebarCollapsed ? 'hidden w-0' : 'w-[260px]']"
    >
      <div class="flex w-[260px] h-full min-h-0 flex-col transition-opacity">
        <div class="h-screen text-white text-base p-2 flex flex-col">
          <div class="flex-1">
            <nav>
              <SidebarNavLink path="/" label="Home" />
              <SidebarNavLink path="/about" label="About" />
              <SidebarNavLink path="/persons" label="Persons" />
            </nav>
          </div>
          <div v-if="user">
            <nav>
              <Menu ref="userMenu" :model="userMenuItems" popup />
              <div
                class="relative group active:opacity-80 p-2 rounded-md cursor-pointer hover:bg-slate-700"
              >
                <a @click="toggleUserMenu">{{ user.name }}</a>
              </div>
            </nav>
          </div>
        </div>
      </div>
    </div>
    <div class="relative flex h-full max-w-full flex-1 flex-col overflow-hidden">
      <main class="relative h-full w-full flex-1 overflow-auto transition-width">
        <div
          class="fixed left-0 top-1/2 z-40 transform -translate-y-1/2 rotate-0 translate-z-0"
          :class="[sidebarCollapsed ? 'translate-x-0' : 'translate-x-64']"
        >
          <button @click="toggleSidebar">
            <span class="group"
              ><div
                class="flex h-[72px] w-8 items-center justify-center transition-all duration-300 opacity-25 hover:opacity-100"
              >
                <div class="flex h-6 w-6 flex-col items-center">
                  <div
                    class="h-3 w-1 rounded-full bg-slate-800 translate-y-[0.15rem] translate-z-0"
                    :class="[sidebarCollapsed ? '-rotate-15' : 'rotate-0 group-hover:rotate-15']"
                  ></div>
                  <div
                    class="h-3 w-1 rounded-full bg-slate-800 -translate-y-[0.15rem]translate-z-0"
                    :class="[sidebarCollapsed ? 'rotate-15' : 'rotate-0 group-hover:-rotate-15']"
                  ></div>
                </div>
              </div>
            </span>
          </button>
        </div>
        <div class="flex h-full flex-col">
          <div class="flex-1 overflow-hidden">
            <div class="flex flex-col pb-9 text-sm">
              <div class="h-full p-8">
                <RouterView />
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { RouterView } from 'vue-router'
import Menu from 'primevue/menu'
import SidebarNavLink from '@/components/SidebarNavLink.vue'
import { authStore } from '../store/auth'

const auth = authStore()

const userMenu = ref()
const userMenuItems = ref([
  {
    label: 'Settings'
  },
  {
    label: 'Logout',
    icon: '',
    command: () => auth.logout()
  }
])
const toggleUserMenu = (event) => {
  userMenu?.value?.toggle(event)
}

const user = computed(() => auth.user)

const sidebarCollapsed = ref(document.documentElement.clientWidth < 1024)

const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

const handleResize = () => {
  sidebarCollapsed.value = document.documentElement.clientWidth < 1024
}

onMounted(() => {
  window.addEventListener('resize', handleResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})
</script>
