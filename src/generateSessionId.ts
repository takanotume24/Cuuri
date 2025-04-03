import { invoke } from "@tauri-apps/api/core";
import { SessionId } from "./types.ts";

export async function generateSessionId(): Promise<SessionId | null> {
  try {
    return await invoke<SessionId>("generate_session_id");
  } catch (error) {
    console.error("Failed to generate session id:", error);
    return null;
  }
}
