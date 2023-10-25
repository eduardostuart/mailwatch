<template>
  <div class="w-full">
    <h2 class="font-semibold text-sm pt-2 mb-2"><slot name="title" /></h2>
    <div class="bg-white rounded-sm shadow text-zinc-900">
      <slot name="body" />
    </div>
  </div>
</template>
