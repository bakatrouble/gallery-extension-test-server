<script setup lang="ts">

import { computed, onMounted, ref } from "vue";
import { useQuery } from "@tanstack/vue-query";
import { fetchIndex, getDownloadUrl, getThumbnailUrl, IndexItem, IndexResponse } from "./api";
import { getIcon } from "material-file-icons";
import Spinner from "./components/Spinner.vue";
import ItemIcon from "./components/ItemIcon.vue";
import ItemTitle from "./components/ItemTitle.vue";
import Breadcrumbs from "./components/Breadcrumbs.vue";

let currentQueryPath = ref("/");

onMounted(() => {
  currentQueryPath.value = window.location.pathname;
})

const { data, isLoading, isError } = useQuery<IndexResponse>({
  queryKey: ['index', currentQueryPath],
  queryFn: async () => {
    const response = await fetchIndex(currentQueryPath.value);
    const dirs = response.items
      .filter((item) => item.is_dir)
      .sort((a, b) => a.name.localeCompare(b.name));
    const files = response.items
      .filter((item) => !item.is_dir)
      .sort((a, b) => a.name.localeCompare(b.name));
    return {
      ...response,
      items: dirs.concat(files),
    };
  }
});

const navigate = async (path: string) => {
  currentQueryPath.value = path;
  window.history.pushState({}, '', path);
}

</script>

<template>
  <div class="@container flex flex-col w-3/4 pt-4 min-h-svh">
    <breadcrumbs
      :root="data?.root_path"
      :path="currentQueryPath"
      @navigate="(path: string) => navigate(path)"
    />
    <div v-if="isLoading" class="grow flex items-center justify-center">
      <spinner />
    </div>
    <div v-else-if="isError">Error!</div>
    <div v-else>
      <div class="grid grid-cols-10 gap-2">
        <a
          class="flex flex-col items-center group"
          v-if="currentQueryPath !== '/'"
          :href="`/${data?.parent_path}`"
          @click.prevent="navigate(`/${data?.parent_path}`)"
        >
          <item-icon :item="{ name: '..', is_dir: true, mime: '', path: '' }" />
          <item-title title=".." />
        </a>
        <a
          v-for="item in data?.items"
          :key="item.path"
          class="flex flex-col items-center group "
          :data-mime="item.mime"
          :data-filename="item.name"
          :href="item.is_dir ? `/${item.path}` : getDownloadUrl(item.path)"
          :target="item.is_dir ? '_self' : '_blank'"
          @click="(e: MouseEvent) => {
            if (item.is_dir) {
              e.preventDefault();
              navigate(`/${item.path}`);
            }
          }"
        >
          <item-icon :item="item" />
          <item-title :title="item.name" />
        </a>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
