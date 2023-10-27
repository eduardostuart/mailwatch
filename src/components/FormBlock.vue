<script setup lang="ts">
import { computed } from "vue";

type Label = {
  value: string;
  for?: string;
};
type Props = {
  label?: Label;
  error?: Record<string, string>;
};

const props = defineProps<Props>();

const hasError = computed(() => {
  return props.error && Object.keys(props.error).length > 0;
});
</script>
<template>
  <div class="w-full py-2 block">
    <label
      v-if="label"
      :for="label?.for"
      class="mb-2 text-xs font-semibold uppercase flex items-center"
      :class="{
        'text-red-600 :': hasError,
        'text-gray-800 dark:text-white': !hasError,
      }"
    >
      {{ label.value }}

      <svg
        v-if="error"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
        fill="currentColor"
        class="w-4 h-4 ml-1"
      >
        <path
          fill-rule="evenodd"
          d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zm0 10a1 1 0 100-2 1 1 0 000 2z"
          clip-rule="evenodd"
        />
      </svg>
    </label>
    <slot />
    <div v-if="hasError" class="pt-1 block text-xs text-red-600">
      <p v-for="err in error">
        {{ err }}
      </p>
    </div>
  </div>
</template>
