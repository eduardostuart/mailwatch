<script setup lang="ts">
// Custom form input
// autocomplete/spellcheck/autocapitaliz is disabled by default
import { computed } from "vue";

type Props = {
  type?: string;
  value?: string;
  modelValue?: string | number;
  autocomplete?: string;
  spellcheck?: boolean | "true" | "false";
  autocorrect?: string;
  autocapitalize?: string;
};

const emits = defineEmits(["update:modelValue"]);

const props = withDefaults(defineProps<Props>(), {
  type: "text",
  value: "",
  autocomplete: "off",
  spellcheck: "false",
  autocorrect: "off",
  autocapitalize: "none",
});

const value = computed({
  get() {
    return props.modelValue;
  },
  set(v) {
    emits("update:modelValue", v);
  },
});
</script>
<template>
  <input
    data-1p-ignore
    :autocomplete="autocomplete"
    :spellcheck="spellcheck"
    :autocorrect="autocorrect"
    :autocapitalize="autocapitalize"
    :value="modelValue"
    :type="type"
    v-model="value"
    class="block w-full px-3 py-2 text-gray-800 dark:text-white bg-gray-100 dark:bg-gray-900 border border-gray-300/90 dark:border-gray-600/60 rounded-md text-sm shadow-sm placeholder-slate-400 focus:outline-none focus:border-pink-500 focus:ring-1 focus:ring-pink-500 disabled:shadow-none invalid:border-red-500 invalid:text-red-600 focus:invalid:border-red-500 focus:invalid:ring-red-500"
  />
</template>
