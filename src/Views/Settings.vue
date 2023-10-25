<script lang="ts" setup>
import { updateSettings, fetchSettings } from "@/Api";
import { FormCheckbox, PreferenceBlock, PreferencePanel } from "@/Components";
import { AppLayout } from "@/Layouts";
import type { Settings } from "@/Models";
import { nextTick, onMounted, reactive, watch } from "vue";
import { useRouter } from "vue-router";

type FormAttrs = {
  showNotifications: boolean;
  showNotificationPreview: boolean;
  notificationSound: boolean;
};

const form = reactive<FormAttrs>({
  showNotifications: false,
  showNotificationPreview: false,
  notificationSound: false,
});

const router = useRouter();
const onCloseClick = () => router.back();

onMounted(async () => {
  const settings = await fetchSettings();
  updateFormValues(settings);
});

function updateFormValues(settings: Settings | null = null) {
  if (settings) {
    form.showNotifications = !!settings.notifications;
    nextTick(() => {
      form.showNotificationPreview = !!settings.preview;
      form.notificationSound = !!settings.sound;
    });
  }
}

let debounce: number | undefined = undefined;
function callUpdateSettings(values: FormAttrs) {
  clearTimeout(debounce);
  debounce = setTimeout(async () => {
    await updateSettings({
      notifications: values.showNotifications,
      sound: !!values.showNotifications ? values.notificationSound : false,
      preview: !!values.showNotifications
        ? values.showNotificationPreview
        : false,
    });
  }, 200);
}

watch<FormAttrs>(form, async (values) => callUpdateSettings(values), {
  immediate: false,
});
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
              <FormCheckbox
                :id="id"
                :checked="form.showNotifications"
                v-model="form.showNotifications"
                class="ml-auto"
              />
            </PreferenceBlock>
            <PreferenceBlock
              label="Sound"
              v-if="form.showNotifications"
              label-for="enable-sound"
              v-slot="{ id }"
            >
              <FormCheckbox
                :id="id"
                :checked="form.notificationSound"
                v-model="form.notificationSound"
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
              <FormCheckbox
                :id="id"
                :checked="form.showNotificationPreview"
                v-model="form.showNotificationPreview"
                class="ml-auto"
              />
            </PreferenceBlock>
          </template>
        </PreferencePanel>
      </div>
    </template>
  </AppLayout>
</template>
@/Components @/Layouts
