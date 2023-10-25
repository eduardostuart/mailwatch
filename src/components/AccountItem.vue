<script lang="ts" setup>
import { computed } from "vue";

type Props = {
  account: Account;
};
const props = defineProps<Props>();

const color = computed(() => `bg-${props.account.color}-600`);
</script>
<template>
  <div
    class="rounded-md p-2 dark:bg-gray-800 hover:dark:bg-gray-900 bg-gray-200 hover:bg-gray-400/80 transition-colors ease-in-out"
  >
    <div class="w-full">
      <h3 class="font-semibold text-base flex items-center">
        <span
          class="w-2 mr-1 h-2 rounded-full p-1 border-2 border-gray-400 dark:border-white/30"
          :class="[color]"
        ></span>
        {{ account.name }}
      </h3>
      <div
        class="text-black dark:text-white/50 text-sm"
        v-if="account.username"
      >
        {{ account.username }}
      </div>
      <div class="text-xs text-black dark:text-white/50">
        {{ account.server }}:{{ account.port }}
      </div>
    </div>
  </div>
</template>
