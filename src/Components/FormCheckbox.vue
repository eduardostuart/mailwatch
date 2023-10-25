<script setup lang="ts">
import { computed } from "vue";

const emit = defineEmits(["update:checked"]);

const props = withDefaults(
  defineProps<{
    id?: string;
    checked?: boolean;
    value?: string | number | boolean;
  }>(),
  {
    checked: false,
  }
);

const proxyChecked = computed({
  get() {
    return props.checked;
  },

  set(val) {
    emit("update:checked", val);
  },
});
</script>

<template>
  <input
    :id="id"
    type="checkbox"
    :value="value"
    v-model="proxyChecked"
    class="rounded transition-all ease-in-out duration-100 cursor-pointer border-zinc-300 text-pink-600 shadow-sm focus:ring-pink-500"
  />
</template>
