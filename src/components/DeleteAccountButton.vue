<script setup lang="ts">
import CustomButton from "./CustomButton.vue";
import { confirm } from "@tauri-apps/api/dialog";
import { api } from "@/api";
import { ref } from "vue";

type Props = {
  id: number;
  onSuccess(): void;
  onError(msg?: string): void;
};

const props = withDefaults(defineProps<Props>(), {
  onSuccess: () => {},
  onError: () => {},
});

const { Account } = api();

const deleting = ref<boolean>(false);
const deleteAccount = async () => {
  if (
    !(await confirm("Confirm account deletion. This action is irreversible.", {
      title: "Confirmation",
      type: "warning",
      okLabel: "DELETE",
      cancelLabel: "Cancel",
    }))
  ) {
    return;
  }

  try {
    await Account.delete(props.id);
    props.onSuccess();
  } catch (err) {
    console.error(err);
    props.onError((err as Error)?.message?.toString());
  }
  deleting.value = false;
};
</script>
<template>
  <CustomButton :loading="deleting" @click.prevent="deleteAccount" type="button"
    ><slot>Delete</slot></CustomButton
  >
</template>
