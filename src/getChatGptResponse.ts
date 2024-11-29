import { invoke } from "@tauri-apps/api/core";

export async function getChatGptResponse(
  currentSessionId: string,
  input: string,
  selectedModel: string,
  apiKey: string
): Promise<string | null> {
  try {
    return await invoke("chat_gpt", {
      inputSessionId: currentSessionId,
      message: input,
      model: selectedModel,
      apiKey: apiKey,
    });
  } catch (error) {
    console.error("Failed to get the response from chatgpt api:", error);
    return null;
  }
}
