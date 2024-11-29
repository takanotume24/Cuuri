import { invoke } from "@tauri-apps/api/core";

export async function getDefaultModel(): Promise<string | null> {
  try {
    return await invoke<string>("get_default_model");
  } catch (error) {
    console.error("Failed to get default model:", error);
    return null;
  }
}
