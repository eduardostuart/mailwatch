<script lang="ts" setup>
import { api } from "@/api";
import {
  CustomButton,
  CustomInput,
  FormBlock,
  CustomColorInput,
  TestConnectionButton,
  DeleteAccountButton,
} from "@/components";
import { Color } from "@/config";
import { AppLayout } from "@/layouts";
import { computed, onMounted, reactive, ref, watch } from "vue";
import { useRouter } from "vue-router";
import { message } from "@tauri-apps/api/dialog";
import { useFormValidation } from "@/composables";

type Form = {
  name: string;
  server: string;
  port: number;
  color: string;
  username: string;
  password: string;
  mailbox: string;
};

const { Account } = api();
const { currentRoute, back: goBack } = useRouter();
const {
  validate,
  rules,
  isValidForm,
  errors: formErrors,
} = useFormValidation();

// Validate only required fields to test the connection
// All other fields are ignored
const canTestConnection = computed(() => {
  if (!formErrors.value) {
    return false;
  }
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

const isCreatingAccount = computed(
  () => currentRoute.value.name === "add-account"
);

const id = computed(() => {
  return parseInt(currentRoute.value.params.id as string, 10) || undefined;
});

const title = computed(() => {
  if (!isCreatingAccount.value) {
    return "Edit account";
  }
  return "Add account";
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

onMounted(async () => {
  if (!isCreatingAccount.value && id.value) {
    const account = await Account.findById(id.value);
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

// Validate the entire for every form change
watch(form, (values) => validateForm(values), {
  immediate: false,
});

const createAccount = async () => {
  await Account.create({
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

const updateAccount = async () => {
  if (!id.value) {
    return;
  }

  await Account.update(id.value, {
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
      await updateAccount();
    } else {
      await createAccount();
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

      <!-- 
        this can be improved. 
        for now, we only test connection for new accounts
      -->
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
