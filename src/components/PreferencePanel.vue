<template>
  <div class="w-full">
    <h2 class="font-semibold text-sm pt-2 pl-2"><slot name="title" /></h2>
    <div class="bg-white rounded-lg shadow text-gray-800">
      <slot name="body" />
    </div>
  </div>
</template>
