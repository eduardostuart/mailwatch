<script lang="ts" setup>
import { useSlots } from "vue";

const slots = useSlots();
type Props = {
  labelFor?: string;
  label: string;
  helpText?: string;
};
withDefaults(defineProps<Props>(), {
  labelFor: "",
});
</script>
<template>
  <div
    class="w-full flex items-center flex-row p-2 border-b border-gray-200 space-x-2"
  >
    <div class="w-full select-none">
      <label :for="labelFor" class="text-xs font-semibold block">{{
        label
      }}</label>
      <p v-if="helpText" class="pt-1 text-xs text-zinc-400">{{ helpText }}</p>
    </div>
    <div class="ml-auto self-start text-right flex-none" v-if="slots.default">
      <slot v-bind="{ id: labelFor }" />
    </div>
  </div>
</template>
