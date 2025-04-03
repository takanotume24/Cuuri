import { invoke } from "npm:@tauri-apps/api/core";
import { ModelName } from "./types.ts";

export async function getDefaultModel(): Promise<ModelName | null> {
  try {
    return await invoke<ModelName>("get_default_model");
  } catch (error) {
    console.error("Failed to get default model:", error);
    return null;
  }
}
