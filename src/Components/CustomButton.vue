<script setup lang="ts">
import LoadingSpinner from "./LoadingSpinner.vue";

type Props = {
  disabled?: boolean;
  loading?: boolean;
};

withDefaults(defineProps<Props>(), {
  disabled: false,
  loading: false,
});
</script>
<template>
  <button
    :class="{
      'bg-pink-800/50 text-pink-200/50 cursor-not-allowed focus:ring-0 focus-within:ring-0':
        disabled,
      'bg-pink-500 text-white hover:bg-pink-600 active:bg-pink-700 focus:ring-pink-300':
        !disabled,
    }"
    class="inline-flex items-center py-1 px-4 uppercase text-xs font-bold rounded-sm focus:outline-none focus:ring"
  >
    <LoadingSpinner class="w-3 h-3 mr-2" v-if="loading" />
    <slot />
  </button>
</template>
