<template>
  <div
    class="relative group active:opacity-80 p-2 rounded-md"
    :class="[isActive ? 'bg-zinc-800' : 'cursor-pointer hover:bg-zinc-800']"
  >
    <RouterLink :to="path"
      ><div>{{ label }}</div></RouterLink
    >
  </div>
</template>

<script setup lang="ts">
import { computed, toRefs } from 'vue'
import { useRoute } from 'vue-router'

const props = defineProps<{
  path: string
  label: string
}>()

const { path } = toRefs(props)

// Access the current route
const route = useRoute()

// Define a computed property for checking if the link is active
const isActive = computed(() => route.path === path.value)
</script>
