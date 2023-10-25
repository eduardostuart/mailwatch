import type { Settings } from "@/Models";
import { invoke } from "@tauri-apps/api";

export async function updateSettings(
  attrs: Settings
): Promise<Settings | null> {
  return invoke<Settings | null>("cmd_update_settings", { attrs });
}

export async function fetchSettings(): Promise<Settings | null> {
  return invoke<Settings | null>("cmd_fetch_settings");
}
