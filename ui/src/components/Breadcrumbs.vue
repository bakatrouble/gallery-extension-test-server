<script setup lang="ts">

import { computed, ref, watch } from "vue";

const { root, path } = defineProps<{
  root?: string,
  path: string,
}>();

const emit = defineEmits<{
  navigate: [path: string],
}>();

const cachedRoot = ref('');

const currentRoot = computed(() => {
  if (root) {
    cachedRoot.value = root;
    return root;
  }
  return cachedRoot.value;
})

const crumbs = computed(() => path.split('/').filter(Boolean));

</script>

<template>
  <h1 class="flex flex-row text-4xl items-end">
    <a
      class="mr-2 text-2xl text-gray-500 dark:text-gray-400 hover:underline"
      v-if="root"
      :href="root"
      @click.prevent="emit('navigate', currentRoot)"
    >
      {{currentRoot}}
    </a>
    <span class="text-gray-500 dark:text-gray-400 mr-2">/</span>
    <template v-for="(crumb, i) in crumbs">
      <a
        class="mr-2 hover:underline"
        :href="`/${crumbs.slice(0, i+1).join('/')}`"
        @click.prevent="emit('navigate', `/${crumbs.slice(0, i+1).join('/')}`)"
      >
        {{crumb}}
      </a>
      <span class="text-gray-500 dark:text-gray-400 mr-2">/</span>
    </template>
  </h1>
</template>

<style scoped>

</style>