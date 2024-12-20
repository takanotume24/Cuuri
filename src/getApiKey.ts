import { invoke } from "@tauri-apps/api/core";
import { ApiKey } from "./types";

export async function getApiKey(): Promise<ApiKey | null> {
  try {
    return await invoke<ApiKey>("get_openai_api_key");
  } catch (error) {
    console.error("Failed to check API key:", error);
    return null;
  }
}
