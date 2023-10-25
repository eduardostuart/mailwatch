<script setup lang="ts">
import { Color } from "@/config";
import { computed, ref } from "vue";

type Props = {
  value?: string;
  modelValue?: string | Color;
};

const emits = defineEmits(["update:modelValue"]);

const colors = computed(() => Object.values(Color));

const props = withDefaults(defineProps<Props>(), {
  value: Color.BLUE,
});

const open = ref<boolean>(false);

const inputValue = ref<string>(props.value);

const selectColor = (color: string) => {
  inputValue.value = color;
  emits("update:modelValue", color);
  open.value = false;
};
</script>
<template>
  <div class="w-full relative min-h-[26px]">
    <div
      @click.prevent="open = !open"
      class="bg-gray-100 dark:bg-gray-900 rounded-md border-gray-300/90 dark:border-gray-600/60 border h-full w-full flex items-center justify-center cursor-pointer"
    >
      <div class="w-4 h-4 rounded-full" :class="`bg-${modelValue}-600`"></div>
    </div>
    <div
      v-if="open"
      class="bg-gray-200 border border-gray-300 dark:bg-gray-800 absolute right-0 top-[100%] grid grid-cols-5 w-[200px] gap-1 p-1 rounded-md shadow-md"
    >
      <a
        v-for="color in colors"
        :key="color"
        class="bg-gray-100 dark:bg-gray-900 hover:border-pink-600 border-gray-300/90 rounded-md dark:border-gray-600/60 border w-9 h-9 flex items-center justify-center"
        href="#"
        @click.prevent="selectColor(color)"
      >
        <div class="w-4 h-4 rounded-full" :class="[`bg-${color}-600`]"></div>
      </a>
    </div>
  </div>
</template>
@/Config/config
