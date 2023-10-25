<script setup lang="ts">
type Props = {
  disabled?: boolean;
};
withDefaults(defineProps<Props>(), {
  disabled: false,
});
</script>
<template>
  <button
    :class="{
      'bg-pink-800/50 text-pink-200/50 cursor-not-allowed': disabled,
      'bg-pink-500 text-white hover:bg-pink-600 active:bg-pink-700 focus:ring-pink-300':
        !disabled,
    }"
    class="py-1 px-6 uppercase text-sm font-bold rounded-sm focus:outline-none focus:ring"
  >
    <slot />
  </button>
</template>
