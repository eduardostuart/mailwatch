import { Event as TauriEvent, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import type { ConnectionCreds } from "@/Models";

export type UnListenConnectionFn = () => Promise<void>;

export async function onTestConnectionResponse(
  cb: (payload: string) => void
): Promise<UnListenConnectionFn> {
  const unlistenFn = await listen<string>(
    "connection_test_result",
    ({ payload }: TauriEvent<string>) => {
      return cb(payload);
    }
  );

  return async (): Promise<void> => {
    if (unlistenFn) {
      unlistenFn();
    }
  };
}

export function testConnection(attrs: ConnectionCreds): Promise<string> {
  return invoke<string>("cmd_test_connection", { attrs });
}
