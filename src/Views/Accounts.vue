<script lang="ts" setup>
import { AccountItem, CustomButton } from "@/Components";
import { AppLayout } from "@/Layouts";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { appWindow } from "@tauri-apps/api/window";
import { allAccounts } from "@/Api";
import type { Account } from "@/Models";
import { AdjustmentsHorizontalIcon } from "@heroicons/vue/20/solid";

const router = useRouter();

const accounts = ref<Account[]>([]);

onMounted(async () => (accounts.value = await allAccounts()));

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
        <AdjustmentsHorizontalIcon class="w-5 h-5" />
      </a>
    </template>
    <template #body>
      <div class="w-full px-4 overflow-y-auto h-full" v-if="accounts.length">
        <AccountItem
          @click.prevent="onAccountItemClick(account)"
          class="cursor-pointer mb-2"
          v-for="account of accounts"
          :key="account.id"
          :account="account"
        />
      </div>
      <div
        v-else
        class="w-full h-full px-4 flex flex-col items-center justify-center"
      >
        <img
          src="/illustration.svg"
          alt="Illustration"
          class="w-[280px] mb-10"
        />
        <p class="text-sm">
          Get started by adding your
          <a
            href="#"
            @click.prevent="onAddAccountClick"
            class="ml-1 underline hover:bg-pink-600 hover:text-white dark:hover:bg-pink-600 dark:text-white dark:hover:text-white rounded-sm transition-colors ease-in-out"
            >first account</a
          >.
        </p>
      </div>
    </template>
    <template #footer>
      <CustomButton @click.prevent="onAddAccountClick" type="button"
        >add account</CustomButton
      >
    </template>
  </AppLayout>
</template>
@/Components @/Layouts @/Api/api
