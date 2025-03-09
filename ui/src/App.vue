<script setup lang="ts">

import { onMounted, ref, watch } from "vue";
import { useQuery } from "@tanstack/vue-query";
import { fetchIndex, fetchServerConfig, getDownloadUrl, type IndexResponse, type ServerConfigResponse } from "./api";
import ItemIcon from "./components/ItemIcon.vue";
import ItemTitle from "./components/ItemTitle.vue";
import Breadcrumbs from "./components/Breadcrumbs.vue";
import alphaSort from "alpha-sort";
import Spinner from "./components/Spinner.vue";

const currentQueryPath = ref("/");
const initialized = ref(false);

onMounted(() => {
  currentQueryPath.value = window.location.pathname;
  window.addEventListener('popstate', () => {
    currentQueryPath.value = window.location.pathname;
  });
  updateTitle();
});

const { data, isLoading, isError, refetch } = useQuery<IndexResponse>({
  queryKey: ['index', currentQueryPath],
  queryFn: async () => {
    const response = await fetchIndex(currentQueryPath.value);
    const dirs = response.items
      .filter((item) => item.is_dir)
      .sort((a, b) => [a.name, b.name].sort(alphaSort({ caseInsensitive: true })).indexOf(a.name) === 0 ? -1 : 1)
    const files = response.items
      .filter((item) => !item.is_dir)
      .sort((a, b) => [a.name, b.name].sort(alphaSort({ caseInsensitive: true })).indexOf(a.name) === 0 ? -1 : 1)
    return {
      ...response,
      items: dirs.concat(files),
    };
  }
});

const { data: serverConfig } = useQuery<ServerConfigResponse>({
  queryKey: ['serverConfig'],
  queryFn: fetchServerConfig,
  refetchInterval: 500,
});

watch(serverConfig, (newConfig, oldConfig) => {
  if (newConfig?.current_path != oldConfig?.current_path && initialized.value) {
    navigate('/');
    refetch();
  }
  initialized.value = true;
});

let updateTitle = () => {
  document.title = `Files - ${currentQueryPath.value}`;
};
watch(currentQueryPath, updateTitle);

const navigate = (path: string) => {
  currentQueryPath.value = path;
  window.history.pushState({}, '', path);
}

</script>

<template>
  <div class="@container flex flex-col w-3/4 pt-4 min-h-svh">
    <breadcrumbs
      :root="serverConfig?.current_path"
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
