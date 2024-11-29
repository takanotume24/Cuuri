import { invoke } from '@tauri-apps/api/core';

export async function getApiKey() {
    try {
      return await invoke<string>('get_openai_api_key');
    } catch (error) {
      console.error('Failed to check API key:', error);
      return null;
    }
}