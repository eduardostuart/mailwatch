<script lang="ts" setup>
import { createAccount, findAccountById, updateAccount } from "@/Api";
import {
  CustomButton,
  CustomInput,
  FormBlock,
  CustomColorInput,
  TestConnectionButton,
  DeleteAccountButton,
} from "@/Components";
import { Color } from "@/Config";
import { AppLayout } from "@/Layouts";
import { computed, onMounted, reactive, ref, watch } from "vue";
import { useRouter } from "vue-router";
import { message } from "@tauri-apps/api/dialog";
import { useFormValidation } from "@/Composables";

type Form = {
  name: string;
  server: string;
  port: number;
  color: string;
  username: string;
  password: string;
  mailbox: string;
};

const { currentRoute, back: goBack } = useRouter();
const {
  validate,
  rules,
  isValidForm,
  errors: formErrors,
} = useFormValidation();

const isCreatingAccount = computed(
  () => currentRoute.value.name === "add-account"
);

const id = computed(() => {
  return parseInt(currentRoute.value.params.id as string, 10) || undefined;
});

const title = computed(() => {
  return !isCreatingAccount.value ? "Edit account" : "Add account";
});

const onConnectionTestEnd = async (result: string) => {
  await message(result, { title: "Connection test", type: "info" });
};

const onConnectionTestFailed = async (err?: string) => {
  await message(err || "Something went wrong", {
    title: "Connection test",
    type: "error",
  });
};

const onDeleteError = async (err?: string) => {
  await message(err || "Something went wrong", {
    title: "Delete account",
    type: "error",
  });
};

const form = reactive<Form>({
  name: "",
  server: "",
  port: 993,
  username: "",
  mailbox: "inbox",
  password: "",
  color: Color.BLUE.toString(),
});

const canTestConnection = computed(() => {
  if (!form.server || !form.port || !form.username || !form.password) {
    return false;
  }
  if (!formErrors.value) {
    return false;
  }
  const errorKeys = Object.keys(formErrors.value);
  return !errorKeys.some((k: string) =>
    ["server", "username", "port", "mailbox", "password"].includes(k)
  );
});

onMounted(async () => {
  if (!isCreatingAccount.value && id.value) {
    const account = await findAccountById(id.value);
    form.name = account?.name || "";
    form.server = account?.server || "";
    form.color = account?.color || Color.BLUE.toString();
    form.username = account?.username || "";
    form.port = account?.port || 993;
    form.mailbox = account?.mailbox || "inbox";
  }
});

const validateForm = (values: Form) => {
  validate(values, {
    name: [rules.required("Name is empty")],
    server: [rules.required("Server is empty")],
    port: [rules.required("Port is empty"), rules.isNumber("Invalid port")],
    username: [rules.required("Username is empty")],
    password: [rules.requiredIf(!id.value, "Password is empty")],
    mailbox: [rules.required("Mailbox is empty")],
  });
};

watch(form, (values) => validateForm(values), {
  immediate: false,
});

const callCreateAccount = async () => {
  await createAccount({
    name: form.name,
    server: form.server,
    port: parseInt(form.port.toString(), 10),
    color: form.color,
    active: true,
    username: form.username,
    password: form.password,
    mailbox: form.mailbox,
  });
};

const callUpdateAccount = async () => {
  if (!id.value) {
    return;
  }

  await updateAccount(id.value, {
    name: form.name,
    server: form.server,
    port: parseInt(form.port.toString(), 10),
    color: form.color,
    active: true,
    username: form.username,
    password: form.password,
    mailbox: form.mailbox,
  });
};

const saving = ref<boolean>(false);
const onFormSubmit = async () => {
  validateForm(form);
  if (!isValidForm.value) {
    return false;
  }
  saving.value = true;
  try {
    if (id.value) {
      await callUpdateAccount();
    } else {
      await callCreateAccount();
    }
    goBack();
  } catch (e) {
    const msg = typeof e === "string" ? e : (e as Error)?.message;
    await message(msg, {
      title: "Error",
      type: "error",
    });
  }
  saving.value = false;
};
</script>
<template>
  <AppLayout
    @keydown.esc="goBack"
    show-close-button
    :on-close-button-click="goBack"
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
          <div class="w-[180px] ml-auto">
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
      <DeleteAccountButton
        :on-success="goBack"
        :on-error="onDeleteError"
        class="ml-2"
        v-if="id"
        :id="id"
      />

      <TestConnectionButton
        :disabled="!canTestConnection"
        :on-test-end="onConnectionTestEnd"
        :on-test-failed="onConnectionTestFailed"
        :mailbox="form.mailbox"
        :server="form.server"
        :username="form.username"
        :password="form.password"
        :port="form.port"
      />
    </template>
  </AppLayout>
</template>
