import type { Account } from "@/Models";
import { invoke } from "@tauri-apps/api";

export function updateAccount(
  id: number,
  attrs: Omit<Account, "id" | "password"> & { password?: string }
): Promise<void> {
  return invoke<void>("cmd_update_account", { id, attrs });
}

export function deleteAccount(id: number): Promise<void> {
  return invoke<void>("cmd_delete_account", { id });
}

export function findAccountById(id: number): Promise<Account | undefined> {
  return invoke<Account | undefined>("cmd_find_account", { id });
}

export function allAccounts(): Promise<Account[]> {
  return invoke<Account[]>("cmd_list_accounts");
}

export function createAccount(attrs: Omit<Account, "id">): Promise<string> {
  return invoke<string>("cmd_create_account", { attrs });
}
