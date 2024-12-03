import { invoke } from "@tauri-apps/api/core";
import { SessionId, UserInput, ModelName, ApiKey, Markdown } from "./types";

export async function getChatGptResponse(
  currentSessionId: SessionId,
  input: UserInput,
  selectedModel: ModelName,
  apiKey: ApiKey
): Promise<Markdown | null> {
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
