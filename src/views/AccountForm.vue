<script lang="ts" setup>
import type { UnlistenFn } from "@tauri-apps/api/event";
import { api } from "@/api";
import {
  CustomButton,
  CustomInput,
  FormBlock,
  CustomColorInput,
} from "@/components";
import { Color } from "@/config";
import { AppLayout } from "@/layouts";
import {
  computed,
  onBeforeMount,
  onBeforeUnmount,
  onMounted,
  reactive,
  ref,
} from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const { Account } = api();

const testing = ref<boolean>(false);

const isCreatingAccount = computed(
  () => router.currentRoute.value.name === "add-account"
);

const id = computed(() => {
  return (
    parseInt(router.currentRoute.value.params.id as string, 10) || undefined
  );
});

const title = computed(() => {
  if (!isCreatingAccount.value) {
    return "Edit account";
  }
  return "Add account";
});

const close = () => router.back();

type Form = {
  name: string;
  server: string;
  port: number;
  color: string;
  username: string;
  password: string;
  mailbox: string;
};

const form = reactive<Form>({
  name: "",
  server: "",
  port: 993,
  username: "",
  mailbox: "mailbox",
  password: "",
  color: Color.BLUE.toString(),
});

onMounted(async () => {
  if (!isCreatingAccount.value && id.value) {
    const account = await Account.findById(id.value);
    form.name = account?.name || "";
    form.server = account?.server || "";
    form.color = account?.color || Color.BLUE.toString();
    form.username = account?.username || "";
    form.port = account?.port || 993;
  }
});

const canSubmit = computed(() => {
  if (
    form.name.trim() === "" ||
    form.server.trim() === "" ||
    String(form.port) === "" ||
    form.username.trim() === "" ||
    form.password.trim() === "" ||
    form.mailbox.trim() === ""
  ) {
    return false;
  }
  return true;
});

const canTestConnection = computed(() => {
  if (
    form.server.trim() === "" ||
    String(form.port) === "" ||
    form.username.trim() === "" ||
    form.password.trim() === "" ||
    form.mailbox.trim() === ""
  ) {
    return false;
  }
  return true;
});

const saving = ref<boolean>(false);
const onFormSubmit = async () => {
  if (!canSubmit.value) {
    return;
  }

  saving.value = true;
  try {
    await Account.create({
      name: form.name,
      server: form.server,
      port: Number(form.port),
      color: form.color,
      active: true,
      username: form.username,
      password: form.password,
      mailbox: form.mailbox,
    });
    close();
  } catch (e) {
    window.alert((e as Error).message);
  }
  saving.value = false;
};

let unListenConnectionTestResult: UnlistenFn | undefined;
onBeforeMount(async () => {
  unListenConnectionTestResult = await Account.onTestConnectionResponse(
    (payload: string) => {
      console.log(payload);
      testing.value = false;
      window.alert(payload);
    }
  );
});

onBeforeUnmount(async () => {
  unListenConnectionTestResult && unListenConnectionTestResult();
});

const onTestConnectionClick = async () => {
  if (!canTestConnection.value) {
    return;
  }
  testing.value = true;
  try {
    await Account.testConnection({
      server: form.server,
      port: form.port,
      username: form.username,
      password: form.password,
      mailbox: form.mailbox,
    });
  } catch (err) {
    console.error(err);
    testing.value = false;
    window.alert(`Error: ${(err as Error).message}`);
  }
};
</script>
<template>
  <AppLayout
    @keydown.esc="close"
    show-close-button
    :on-close-button-click="close"
  >
    <template #title>{{ title }}</template>
    <template #body>
      <form
        @submit.prevent="onFormSubmit"
        class="w-full px-4 overflow-y-auto h-full"
      >
        <button type="submit" class="hidden" />
        <div class="w-full flex flex-row">
          <div class="w-full">
            <FormBlock :label="{ value: 'Name', for: 'name' }">
              <CustomInput
                tabindex="0"
                v-model="form.name"
                id="name"
                type="text"
                placeholder="Name"
              />
            </FormBlock>
          </div>
          <div class="w-[40px] ml-4">
            <FormBlock :label="{ value: 'Color', for: 'color' }">
              <CustomColorInput
                tabindex="0"
                class="h-[38px]"
                v-model="form.color"
              />
            </FormBlock>
          </div>
        </div>
        <div class="w-full flex flex-row">
          <div class="w-[100%] mr-6">
            <FormBlock :label="{ value: 'Server', for: 'server' }">
              <CustomInput
                v-model="form.server"
                id="server"
                type="text"
                placeholder="Server"
              />
            </FormBlock>
          </div>
          <div class="w-[100px] ml-auto">
            <FormBlock :label="{ value: 'Port', for: 'port' }">
              <CustomInput
                v-model="form.port"
                id="port"
                inputmode="numeric"
                pattern="\d*"
                type="text"
                value="993"
              />
            </FormBlock>
          </div>
        </div>
        <div class="w-full flex flex-row">
          <div class="w-[50%] mr-6">
            <FormBlock :label="{ value: 'Username', for: 'username' }">
              <CustomInput
                v-model="form.username"
                id="username"
                type="text"
                placeholder="email@domain.com"
              />
            </FormBlock>
          </div>
          <div class="w-[50%] ml-auto">
            <FormBlock :label="{ value: 'Password', for: 'password' }">
              <CustomInput
                v-model="form.password"
                id="password"
                type="password"
              />
            </FormBlock>
          </div>
        </div>
        <div class="w-full">
          <FormBlock :label="{ value: 'mailbox', for: 'mailbox' }">
            <CustomInput v-model="form.mailbox" name="mailbox" id="mailbox" />
          </FormBlock>
        </div>
      </form>
    </template>
    <template #footer>
      <CustomButton
        :disabled="!canSubmit"
        :loading="saving"
        @click.prevent="onFormSubmit"
        type="button"
        >Save</CustomButton
      >
      <CustomButton
        @click.prevent="onTestConnectionClick"
        class="ml-auto"
        :loading="testing"
        :disabled="testing || !canTestConnection"
        type="button"
      >
        test connection</CustomButton
      >
    </template>
  </AppLayout>
</template>
