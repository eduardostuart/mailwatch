import { invoke } from "@tauri-apps/api";
import { Event as TauriEvent, UnlistenFn, listen } from "@tauri-apps/api/event";

export function api() {
  return {
    Account: {
      // Listener to connection test response
      onTestConnectionResponse(
        cb: (payload: string) => void
      ): Promise<UnlistenFn> {
        return listen<string>(
          "connection_test_result",
          ({ payload }: TauriEvent<string>) => {
            return cb(payload);
          }
        );
      },
      // Execute test connection command
      testConnection(attrs: ConnectionCreds): Promise<string> {
        return invoke<string>("cmd_test_connection", { attrs });
      },
      update(
        id: number,
        attrs: Omit<Account, "id" | "password"> & { password?: string }
      ): Promise<void> {
        return invoke<void>("cmd_update_account", { id, attrs });
      },
      delete(id: number): Promise<void> {
        return invoke<void>("cmd_delete_account", { id });
      },
      // Find an account by id
      findById(id: number): Promise<Account | undefined> {
        return invoke<Account | undefined>("cmd_find_account", { id });
      },
      // Retrieve all accounts
      all(): Promise<Account[]> {
        return invoke<Account[]>("cmd_list_accounts");
      },
      // Create new account in the db
      create(attrs: Omit<Account, "id">): Promise<string> {
        return invoke<string>("cmd_create_account", { attrs });
      },
    },
  };
}
