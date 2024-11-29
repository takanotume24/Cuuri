import { invoke } from "@tauri-apps/api/core";

export async function getChatHistory(): Promise<Array<{
  session_id: string;
  question: string;
  answer: string;
}> | null> {
  try {
    return await invoke("get_chat_history");
  } catch (error) {
    console.error("Failed to get chat history:", error);
    return null;
  }
}
