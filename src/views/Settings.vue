<script lang="ts" setup>
import { PreferenceBlock, PreferencePanel } from "@/components";
import { AppLayout } from "@/layouts";
import { reactive } from "vue";
import { useRouter } from "vue-router";

type FormAttrs = {
  showNotifications: boolean;
  showNotificationPreview: boolean;
  notificationSound: boolean;
};

const form = reactive<FormAttrs>({
  showNotifications: true,
  showNotificationPreview: false,
  notificationSound: false,
});

// TODO: save...

const router = useRouter();
const onCloseClick = () => router.back();
</script>
<template>
  <AppLayout
    @keydown.esc="onCloseClick"
    show-close-button
    :on-close-button-click="onCloseClick"
  >
    <template #title>Settings</template>
    <template #body>
      <div class="w-full px-4 overflow-y-auto h-full">
        <PreferencePanel>
          <template #title> Notifications </template>
          <template #body>
            <PreferenceBlock
              label="Show notifications"
              label-for="show-notifications"
              v-slot="{ id }"
            >
              <input
                type="checkbox"
                v-model="form.showNotifications"
                :id="id"
                class="ml-auto"
              />
            </PreferenceBlock>
            <PreferenceBlock
              label="Sound"
              v-if="form.showNotifications"
              label-for="enable-sound"
              v-slot="{ id }"
            >
              <input
                type="checkbox"
                v-model="form.notificationSound"
                :id="id"
                class="ml-auto"
              />
            </PreferenceBlock>
            <PreferenceBlock
              label="Show message preview"
              label-for="show-preview"
              v-if="form.showNotifications"
              help-text="Preview email subject inside new message notifications"
              v-slot="{ id }"
            >
              <input
                type="checkbox"
                v-model="form.showNotificationPreview"
                :id="id"
                class="ml-auto"
              />
            </PreferenceBlock>
          </template>
        </PreferencePanel>
      </div>
    </template>
  </AppLayout>
</template>
