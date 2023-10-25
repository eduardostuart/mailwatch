<script setup lang="ts">
import { computed } from "vue";

export type Option = {
  value: String;
  label: String;
};

type Props = {
  selected?: string;
  options?: Option[];
  modelValue?: string | number;
};

const emits = defineEmits(["update:modelValue"]);

const props = withDefaults(defineProps<Props>(), {
  selected: "",
});

const value = computed({
  get() {
    return props.selected;
  },
  set(v) {
    emits("update:modelValue", v);
  },
});
</script>
<template>
  <select
    data-1p-ignore
    v-model="value"
    class="block w-full px-3 py-2 text-white bg-gray-900 border border-gray-600/60 rounded-md text-sm shadow-sm placeholder-slate-400 focus:outline-none focus:border-pink-500 focus:ring-1 focus:ring-pink-500 disabled:shadow-none invalid:border-red-500 invalid:text-red-600 focus:invalid:border-red-500 focus:invalid:ring-red-500"
  >
    <option
      :selected="modelValue === option.value"
      :value="option.value"
      v-for="option in options"
    >
      {{ option.label }}
    </option>
  </select>
</template>
