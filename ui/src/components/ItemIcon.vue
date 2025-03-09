<script setup lang="ts">
import { getThumbnailUrl, type IndexItem } from "../api.js";
import { getIcon } from "material-file-icons";
import { computed, ref, useTemplateRef } from "vue";
import { useElementVisibility } from "@vueuse/core";
import Spinner from "./Spinner.vue";

const { item } = defineProps<{
  item: IndexItem,
}>();

const thumbnailAllowedMimes = [
  "image/jpeg",
  "image/png",
  "image/webp",
  "image/gif",
];

const shouldGenerateThumbnail = computed(() => {
  return thumbnailAllowedMimes.includes(item.mime);
});

const isLoading = ref(true);

const visibilityTarget = useTemplateRef<HTMLImageElement>('visibilityTarget');
const isVisible = useElementVisibility(visibilityTarget, {
  rootMargin: '100px 0px'
});

const icon = computed(() => {
  if (item.is_dir) {
    return `<svg
        xmlns="http://www.w3.org/2000/svg"
        style="width: 100%; height: 100%"
        viewBox="0 0 24 24"
      >
        <path
          d="M10,4H4C2.89,4 2,4.89 2,6V18A2,2 0 0,0 4,20H20A2,2 0 0,0 22,18V8C22,6.89 21.1,6 20,6H12L10,4Z" fill="#42a5f5"
        />
      </svg>`;
  } else {
      return getIcon(item.name).svg;
  }
});
</script>

<template>
  <div class="m-2.5 mb-1 relative w-full">
    <div
      class="relative aspect-square flex items-center justify-center"
      v-if="shouldGenerateThumbnail"
    >
      <img
        ref="visibilityTarget"
        :class="[
          'max-w-full max-h-full inset-0 rounded-sm top-0 left-0 object-contain',
          isLoading ? 'opacity-0' : 'opacity-100',
        ]"
        :src="!isVisible ? 'data:null' : getThumbnailUrl(item.path)"
        @load="isLoading = false"
      />
      <div
        class="absolute left-0 top-0 right-0 bottom-0 flex items-center justify-center"
        v-if="isLoading"
      >
        <spinner />
      </div>
    </div>
    <div
      class="w-full p-2 h-full inset-0"
      v-else
      v-html="icon"
    />
  </div>
</template>

<style scoped>

</style>