<script setup lang="ts">
const { title } = defineProps<{
  title: string
}>();
</script>

<template>
  <div class="title-container relative max-w-full">
    <p class="
      text-xs overflow-clip text-nowrap text-ellipsis text-center
      max-w-full
    ">
      {{ title }}
    </p>
    <p class="
      text-xs break-all text-center
      absolute -top-0.5 -left-1 -right-1
      px-1 py-0.5
      opacity-0 group-hover:opacity-100
      pointer-events-none group-hover:pointer-events-auto
      dark:bg-gray-700
      rounded-sm
      z-10
    ">
      {{ title }}
    </p>
  </div>
</template>

<style scoped>

</style>
