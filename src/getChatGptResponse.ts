import { invoke } from "@tauri-apps/api/core";
import { SessionId, UserInput, ModelName, ApiKey, Markdown } from "./types";

export async function getChatGptResponse(
  currentSessionId: SessionId,
  input: UserInput,
  selectedModel: ModelName,
  apiKey: ApiKey,
  base64Image?: string
): Promise<Markdown | null> {
  try {
    return await invoke("get_chatgpt_response", {
      inputSessionId: currentSessionId,
      message: input,
      model: selectedModel,
      apiKey: apiKey,
      base64Image: base64Image,
    });
  } catch (error) {
    console.error("Failed to get the response from chatgpt api:", error);
    return null;
  }
}
