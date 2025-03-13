// src/streamChatGptResponse.ts (あるいは getChatGptResponse.ts の修正版)
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  SessionId,
  UserInput,
  ModelName,
  ApiKey,
  ChatResponse,
  EncodedImage,
} from "./types";

/**
 * ChatGPTへの問い合わせをストリーミングで行い、
 * トークンごとにUI更新するサンプル。
 */
export async function streamChatGptResponse(
  currentSessionId: SessionId,
  input: UserInput,
  selectedModel: ModelName,
  apiKey: ApiKey,
  onToken: (partialText: string) => void, // 部分テキストを受け取った時のコールバック
  base64ImageList?: EncodedImage[]
): Promise<ChatResponse | null> {
  try {
    // 1) 部分更新のためのイベント受信をセットアップ
    let unlisten = await listen("token", (event) => {
      const tokenChunk = event.payload as string;
      // ここで受け取った文字列をコールバックでUIなどに反映
      onToken(tokenChunk);
    });

    // 2) Rust 側コマンドを呼ぶ。完了すると最終的な ChatResponse が返ってくる
    const finalResponse = (await invoke("stream_chatgpt_response", {
      inputSessionId: currentSessionId,
      message: input,
      model: selectedModel,
      apiKey: apiKey,
      base64Images: base64ImageList,
    })) as ChatResponse;

    // イベント受信を解除
    unlisten();

    return finalResponse;
  } catch (error) {
    console.error("Error during streaming:", error);
    return null;
  }
}
