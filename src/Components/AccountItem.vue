<script lang="ts" setup>
import type { Account } from "@/Models";
import { computed } from "vue";

type Props = {
  account: Account;
};
const props = defineProps<Props>();

const color = computed(() => `bg-${props.account.color}-600`);
</script>
<template>
  <div
    class="rounded-md p-2 dark:bg-zinc-800 hover:dark:bg-zinc-900 bg-zinc-200 hover:bg-zinc-400/80 transition-colors ease-in-out"
  >
    <div class="w-full">
      <h3 class="font-semibold text-base flex items-center">
        <span class="w-3 mr-1 h-3 rounded-full p-1" :class="[color]"></span>
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
