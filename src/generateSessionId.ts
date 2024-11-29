import { invoke } from "@tauri-apps/api/core";

export async function generateSessionId(): Promise<string | null> {
  try {
    return await invoke<string>("generate_session_id");
  } catch (error) {
    console.error("Failed to generate session id:", error);
    return null;
  }
}
