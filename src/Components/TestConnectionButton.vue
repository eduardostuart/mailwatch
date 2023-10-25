<script lang="ts" setup>
import type { UnListenConnectionFn } from "@/Api";
import { onTestConnectionResponse, testConnection } from "@/Api";
import { onBeforeMount, onBeforeUnmount, ref } from "vue";
import CustomButton from "./CustomButton.vue";

type Props = {
  server: string;
  port: number;
  username: string;
  password: string;
  mailbox: string;
  disabled: boolean;
  onTestEnd?: (result: string) => void | Promise<void>;
  onTestFailed?: (msg?: string) => void | Promise<void>;
};

const props = withDefaults(defineProps<Props>(), {
  disabled: true,
  onTestEnd: (_: string) => {},
  onTestFailed: (_?: string) => {},
});

// Keep state of connection test
// Test runs async, it should change the state in case of failure
// or when the test is done
const testing = ref<boolean>(false);

let unListenTest: UnListenConnectionFn;
onBeforeMount(async () => {
  unListenTest = await onTestConnectionResponse(async (result: string) => {
    testing.value = false;
    await props.onTestEnd(result);
  });
});

onBeforeUnmount(() => unListenTest());

const runTest = async () => {
  if (props.disabled) {
    return;
  }
  testing.value = true;
  try {
    await testConnection({
      server: props.server,
      port: props.port,
      username: props.username,
      password: props.password,
      mailbox: props.mailbox,
    });
  } catch (err) {
    const msg = (err as Error)?.message || err?.toString();
    await props.onTestFailed(msg);
    testing.value = false;
  }
};
</script>
<template>
  <CustomButton
    @click.prevent="runTest"
    class="ml-auto"
    :loading="testing"
    :disabled="testing || disabled"
    type="button"
  >
    <slot>test connection</slot>
  </CustomButton>
</template>
@/Api/api
