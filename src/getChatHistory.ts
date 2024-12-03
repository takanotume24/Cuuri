import { invoke } from "@tauri-apps/api/core";
import { DatabaseChatEntry } from "./types";

export async function getChatHistory(): Promise<Array<DatabaseChatEntry> | null> {
  try {
    return await invoke("get_chat_history");
  } catch (error) {
    console.error("Failed to get chat history:", error);
    return null;
  }
}
