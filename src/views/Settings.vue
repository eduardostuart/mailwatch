<script lang="ts" setup>
import { AppLayout } from "@/layouts";
import { useRouter } from "vue-router";

const router = useRouter();
const onCloseClick = () => router.back();
</script>
<template>
  <AppLayout
    @keydown.esc="onCloseClick"
    show-close-button
    :on-close-button-click="onCloseClick"
  >
    <template #title>Settings</template>
    <template #body>
      <div class="w-full px-4 overflow-y-auto h-full">TODO :)</div>
    </template>
  </AppLayout>
</template>
