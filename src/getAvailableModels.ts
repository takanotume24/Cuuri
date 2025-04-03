import { invoke } from "npm:@tauri-apps/api/core";

export async function getAvailableModels(
  apiKey: string
): Promise<string[] | null> {
  try {
    return await invoke<string[]>("get_available_models", {
      apiKey: apiKey,
    });
  } catch (error) {
    console.error("Failed to get Available models:", error);
    return null;
  }
}
