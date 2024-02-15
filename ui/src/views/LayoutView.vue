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
              <SidebarNavLink
                path="/schools"
                :label="$t('views.schools')"
                v-if="authStore.canSee('/schools')"
              />
              <SidebarNavLink
                :path="`/schools/${selectedSchool?.id.toString()}/years`"
                :label="$t('views.years')"
                v-if="authStore.canSee('/years') && selectedSchool"
              />
              <SidebarNavLink
                :path="`/schools/${selectedSchool?.id.toString()}/streams`"
                :label="$t('views.streams')"
                v-if="authStore.canSee('/streams') && selectedSchool"
              />
              <SidebarNavLink
                :path="`/schools/${selectedSchool?.id.toString()}/streams/${selectedStream?.id.toString()}/groups`"
                :label="$t('views.groups')"
                v-if="authStore.canSee('/groups') && selectedSchool && selectedStream"
              />
              <SidebarNavLink path="/persons" label="Persons" v-if="authStore.canSee('/persons')" />
            </nav>
          </div>
          <div v-if="user">
            <nav>
              <Menu ref="userMenu" :model="userMenuItems" popup />
              <div
                class="relative group active:opacity-80 p-2 rounded-md cursor-pointer hover:bg-zinc-800"
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
                    class="h-3 w-1 rounded-full bg-slate-800 dark:bg-white translate-y-[0.15rem] translate-z-0"
                    :class="[sidebarCollapsed ? '-rotate-15' : 'rotate-0 group-hover:rotate-15']"
                  ></div>
                  <div
                    class="h-3 w-1 rounded-full bg-slate-800 dark:bg-white -translate-y-[0.15rem]translate-z-0"
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
              <div class="pt-4 pl-8 mb-2 border-none">
                <Menubar
                  :model="breadcrumbs"
                  :pt="{
                    root: ({}) => ({
                      class: 'border-none'
                    }),
                    content: ({ context }) => ({
                      class:
                        'transition-shadow duration-200 rounded-md text-gray-700 dark:text-white/80 hover:text-gray-700 dark:hover:text-white/80 ' +
                        (context.level === 0 && context.index % 2 === 1
                          ? ''
                          : 'hover:bg-gray-200 ' +
                            (context.level === 0
                              ? 'dark:hover:bg-gray-900/80'
                              : 'dark:hover:bg-gray-800/80'))
                    }),
                    action: ({ context }) => ({
                      class:
                        'flex align-items-center select-none flex items-center no-underline overflow-hidden relative py-1 px-2 ' +
                        (context.level === 0 && context.index % 2 === 1 ? '' : 'cursor-pointer')
                    })
                  }"
                >
                  <template #item="{ item, props, hasSubmenu }">
                    <a disabled class="flex align-items-center" v-bind="props.action">
                      <span class="ml-2">{{ item.label }}</span>
                      <i v-if="hasSubmenu" class="pi pi-angle-down ml-2"></i>
                    </a>
                  </template>
                </Menubar>
              </div>
              <div class="h-full p-8" style="font-size: 14px !important">
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
import { RouterView, useRouter } from 'vue-router'
import Menu from 'primevue/menu'
import SidebarNavLink from '@/components/SidebarNavLink.vue'
import { useAuthStore } from '../store/auth'
import { useSchoolsStore } from '../store/schools'
import { useStreamsStore } from '../store/streams'
import { useLayoutStore } from '../store/layout'
import Menubar from 'primevue/menubar'
import { useRoute } from 'vue-router'
import type School from '@/models/School'
import type Stream from '@/models/Stream'

const authStore = useAuthStore()
const schoolsStore = useSchoolsStore()
const streamsStore = useStreamsStore()
const layoutStore = useLayoutStore()
const route = useRoute()
const router = useRouter()

const selectedSchool = ref(layoutStore.school)
const selectedStream = ref(layoutStore.stream)
const allSchools = ref<School[]>([])
const allStreams = ref<Stream[]>([])

const breadcrumbs = computed(() => [
  {
    label: selectedSchool.value?.name,
    items: allSchools.value.map((s) => {
      return {
        label: s.name,
        command: changeSchool(s)
      }
    }),
    visible: route.meta.breadcrumbsLevel !== undefined && route.meta.breadcrumbsLevel > 0
  },
  {
    label: '/',
    visible: route.meta.breadcrumbsLevel !== undefined && route.meta.breadcrumbsLevel > 1
  },
  {
    label: selectedStream.value?.name,
    items: allStreams.value.map((s) => {
      return {
        label: s.name,
        command: changeStream(s)
      }
    }),
    visible: route.meta.breadcrumbsLevel !== undefined && route.meta.breadcrumbsLevel > 1
  }
])

const userMenu = ref()
const userMenuItems = ref([
  {
    label: 'Settings'
  },
  {
    label: 'Logout',
    icon: '',
    command: () => authStore.logout()
  }
])
const toggleUserMenu = (event) => {
  userMenu?.value?.toggle(event)
}

const changeSchool = (s: School) => async () => {
  selectedSchool.value = s
  layoutStore.setSchool(s)

  await refreshStream()

  if (route.params.schoolId) {
    const newRoute = { ...route, path: '', params: { ...route.params, schoolId: s.id } }
    router.push(newRoute)
  }
}

const changeStream = (s: Stream) => () => {
  selectedStream.value = s
  layoutStore.setStream(s)

  if (route.params.streamId) {
    const newRoute = { ...route, path: '', params: { ...route.params, streamId: s.id } }
    router.push(newRoute)
  }
}

const user = computed(() => authStore.user)

const sidebarCollapsed = ref(document.documentElement.clientWidth < 1024)

const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

const handleResize = () => {
  sidebarCollapsed.value = document.documentElement.clientWidth < 1024
}

const refreshStream = async () => {
  if (selectedSchool.value) {
    await streamsStore.fetchStreams(selectedSchool.value.id)

    allStreams.value = streamsStore.streams

    // Reset saved stream if it's either empty or set to a non-exiting value
    if (streamsStore.streams.length > 0) {
      console.log('Checking selected stream')
      if (
        layoutStore.stream === null ||
        streamsStore.streams.findIndex((s) => s.id === layoutStore.stream?.id) === -1
      ) {
        layoutStore.setStream(streamsStore.streams[0])
        selectedStream.value = layoutStore.stream
      }
    } else {
      console.log('Setting stream to null')

      layoutStore.resetStream()
    }
  } else {
    layoutStore.resetStream()
  }
}

onMounted(async () => {
  window.addEventListener('resize', handleResize)

  await schoolsStore.fetchSchools()

  allSchools.value = schoolsStore.schools

  // Reset saved school if it's either empty or set to a non-exiting value
  if (schoolsStore.schools.length > 0) {
    console.log('Checking selected school')
    if (
      layoutStore.school === null ||
      schoolsStore.schools.findIndex((s) => s.id === layoutStore.school?.id) === -1
    ) {
      layoutStore.setSchool(schoolsStore.schools[0])
      selectedSchool.value = layoutStore.school
    }
  } else {
    console.log('Setting school to null')

    layoutStore.resetSchool()
  }

  await refreshStream()
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})
</script>
