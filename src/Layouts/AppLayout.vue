<script setup lang="ts">
import { onMounted, ref, useSlots } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { XMarkIcon } from "@heroicons/vue/20/solid";

const slots = useSlots();

type Props = {
  showCloseButton: boolean;
  onCloseButtonClick: ((...args: any[]) => void) | undefined;
};

withDefaults(defineProps<Props>(), {
  showCloseButton: false,
  onCloseButtonClick: () => {},
});

const layout = ref<HTMLDivElement>();
onMounted(() => layout.value?.focus());

const onDrag = async () => {
  await appWindow.startDragging();
};
</script>
<template>
  <div
    tabindex="-1"
    ref="layout"
    class="w-full focus:outline-none h-full overflow-hidden flex flex-col"
  >
    <header
      v-if="slots.title"
      @mousedown="onDrag"
      data-tauri-drag-region
      class="w-full h-[45px] select-none flex-none mb-2 border-b dark:border-zinc-900/80 dark:border-zinc-50"
    >
      <div class="flex items-center w-full h-full px-4">
        <XMarkIcon
          @click.prevent="onCloseButtonClick"
          title="close"
          class="mr-2 w-5 h-5 cursor-pointer"
          v-if="showCloseButton"
        />
        <h1 class="font-semibold w-full text-center text-lg">
          <slot name="title" />
        </h1>
        <slot name="headeroptions" />
      </div>
    </header>
    <div class="w-full h-full overflow-hidden text-zinc-800 dark:text-white">
      <slot name="body" />
    </div>

    <div
      v-if="slots.footer"
      class="w-full h-[46px] py-2 text-sm border-zinc-200 dark:bg-black border-t dark:border-zinc-600/30"
    >
      <div class="px-4 w-full h-full flex items-center">
        <slot name="footer" />
      </div>
    </div>
  </div>
</template>
@/Components
