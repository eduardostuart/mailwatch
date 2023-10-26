<script lang="ts" setup>
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
  <div class="w-full flex flex-row p-2 border-b border-gray-200">
    <div class="w-[50%] select-none">
      <label :for="labelFor" class="text-sm block">{{ label }}</label>
      <p v-if="helpText" class="text-xs text-gray-400">{{ helpText }}</p>
    </div>
    <div class="ml-auto text-right w-[50%]">
      <slot v-bind="{ id: labelFor }" />
    </div>
  </div>
</template>
