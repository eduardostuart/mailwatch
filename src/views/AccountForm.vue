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
  watch,
} from "vue";
import { useRouter } from "vue-router";
import { confirm, message } from "@tauri-apps/api/dialog";
import { useFormValidation } from "@/composables";

const router = useRouter();
const {
  validate,
  rules,
  isValidForm,
  errors: formErrors,
} = useFormValidation();

const { Account } = api();

// Validate only required fields to test the connection
// All other fields are ignored
const canTestConnection = computed(() => {
  const errorKeys = Object.keys(formErrors.value);
  if (
    errorKeys.some((k: string) =>
      ["server", "username", "port", "mailbox", "password"].includes(k)
    )
  ) {
    return false;
  }
  return true;
});

// Keep state of connection test
// Test runs async, it should change the state in case of failure
// or when the test is done
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

// "Close" current window and move back to initial page
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

const validateForm = (values: Form) => {
  validate(values, {
    name: [rules.required("Name is empty")],
    server: [rules.required("Server is empty")],
    port: [rules.required("Port is empty")],
    username: [rules.required("Username is empty")],
    password: [rules.required("Password is empty")],
    mailbox: [rules.required("Mailbox is empty")],
  });
};

// Validate the entire for every form change
watch(form, (values) => validateForm(values));

const saving = ref<boolean>(false);
const onFormSubmit = async () => {
  validateForm(form);
  if (!isValidForm.value) {
    return false;
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
    await message((e as Error).message, {
      title: "Error",
      type: "error",
    });
  }
  saving.value = false;
};

let unListenConnectionTestResult: UnlistenFn | undefined;
onBeforeMount(async () => {
  unListenConnectionTestResult = await Account.onTestConnectionResponse(
    async (payload: string) => {
      testing.value = false;
      await message(payload, {
        title: "Test connection",
        type: "info",
      });
    }
  );
});

onBeforeUnmount(async () => {
  unListenConnectionTestResult && unListenConnectionTestResult();
});

const onTestConnectionClick = async () => {
  validateForm(form);
  if (!canTestConnection) {
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
    await message((err as Error).message, {
      title: "Error",
      type: "error",
    });
  }
};

const deleting = ref<boolean>(false);
const onDeleteClick = async () => {
  if (!id.value) {
    return;
  }

  const result = await confirm(
    "Confirm account deletion. This action is irreversible.",
    {
      title: "Confirmation",
      type: "warning",
      okLabel: "DELETE",
      cancelLabel: "Cancel",
    }
  );

  if (!result) {
    return;
  }

  try {
    await Account.delete(id.value);
    close();
  } catch (err) {
    console.error(err);
  }
  deleting.value = false;
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
        autocomplete="false"
        @submit.prevent="onFormSubmit"
        class="w-full px-4 overflow-y-auto h-full"
      >
        <button type="submit" class="hidden" />
        <div class="w-full flex flex-row">
          <div class="w-full">
            <FormBlock
              :error="formErrors?.name"
              :label="{ value: 'Name', for: 'name' }"
            >
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
            <FormBlock
              :error="formErrors?.server"
              :label="{ value: 'Server', for: 'server' }"
            >
              <CustomInput
                v-model="form.server"
                id="server"
                type="text"
                placeholder="Server"
              />
            </FormBlock>
          </div>
          <div class="w-[100px] ml-auto">
            <FormBlock
              :error="formErrors?.port"
              :label="{ value: 'Port', for: 'port' }"
            >
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
            <FormBlock
              :error="formErrors?.username"
              :label="{ value: 'Username', for: 'username' }"
            >
              <CustomInput
                v-model="form.username"
                id="username"
                type="text"
                placeholder="email@domain.com"
              />
            </FormBlock>
          </div>
          <div class="w-[50%] ml-auto">
            <FormBlock
              :error="formErrors?.password"
              :label="{ value: 'Password', for: 'password' }"
            >
              <CustomInput
                v-model="form.password"
                id="password"
                type="password"
              />
            </FormBlock>
          </div>
        </div>
        <div class="w-full">
          <FormBlock
            :error="formErrors?.mailbox"
            :label="{ value: 'mailbox', for: 'mailbox' }"
          >
            <CustomInput v-model="form.mailbox" name="mailbox" id="mailbox" />
          </FormBlock>
        </div>
      </form>
    </template>
    <template #footer>
      <CustomButton
        :disabled="!isValidForm"
        :loading="saving"
        @click.prevent="onFormSubmit"
        type="button"
        >Save</CustomButton
      >
      <CustomButton
        v-if="id"
        class="ml-2"
        :loading="deleting"
        @click.prevent="onDeleteClick"
        type="button"
        >Delete</CustomButton
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
