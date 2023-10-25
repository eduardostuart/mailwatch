<script lang="ts" setup>
import { AccountItem, CustomButton } from "@/components";
import { AppLayout } from "@/layouts";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { appWindow } from "@tauri-apps/api/window";
import { api } from "@/api";

const router = useRouter();
const { Account } = api();

const accounts = ref<Account[]>([]);

onMounted(async () => {
  accounts.value = await Account.all();
});

const onAccountItemClick = (account: Account) => {
  router.push({ name: "edit-account", params: { id: account.id } });
};
const onAddAccountClick = () => router.push({ name: "add-account" });
const onCloseClick = () => appWindow.hide();
</script>
<template>
  <AppLayout
    @keydown.esc="onCloseClick"
    show-close-button
    :on-close-button-click="onCloseClick"
  >
    <template #title>Accounts</template>
    <template #headeroptions>
      <a
        href="#"
        title="Settings"
        class="text-gray-800 dark:text-white ml-auto"
        @click.prevent="router.push({ name: 'settings' })"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="w-6 h-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M10.5 6h9.75M10.5 6a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-9.75 0h9.75"
          />
        </svg>
      </a>
    </template>
    <template #body>
      <div class="w-full px-4 overflow-y-auto h-full">
        <AccountItem
          @click.prevent="onAccountItemClick(account)"
          class="cursor-pointer mb-2"
          v-for="account of accounts"
          :key="account.id"
          :account="account"
        />
      </div>
    </template>
    <template #footer>
      <CustomButton @click.prevent="onAddAccountClick" type="button"
        >add account</CustomButton
      >
    </template>
  </AppLayout>
</template>
