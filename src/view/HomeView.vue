<template>
  <div id="app" class="d-flex vh-100">
    <!-- Left side: Session list and Settings button -->
    <aside class="col-3 overflow-auto border-end">
      <ChatSessions v-model:currentSessionId="currentSessionId" :isApiKeySet="isApiKeySet"
        v-model:selectedModel="selectedModel" />
      <div class="d-flex justify-content-end mt-3">
        <button class="btn btn-primary" @click="goToSettings">Settings</button>
      </div>
    </aside>

    <!-- Right side: Chat history + Input form -->
    <main class="col-9 d-flex flex-column p-3">
      <header class="flex-grow-1 overflow-auto mb-3">
        <!-- Component for displaying chat history (streamingAnswer etc.) -->
        <ChatHistory :currentSessionId="currentSessionId" :lastAnswerReceivedTime="lastAnswerReceivedTime"
          :streamingAnswer="partialAnswer" :lastUserQuestion="lastUserQuestion" />
      </header>

      <!-- Footer input form -->
      <footer class="mt-auto">
        <ChatInputForm :onSubmit="handleSubmit" />
      </footer>
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, nextTick } from 'vue';
import { useRouter } from 'vue-router';
import dayjs from 'dayjs';

// Components
import ChatSessions from '../components/ChatSessions.vue';
import ChatHistory from '../components/ChatHistory.vue';
import ChatInputForm from '../components/ChatInputForm.vue';

// Type definitions and utilities
import { SessionId, ModelName, EncodedImage } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getApiKey } from '../getApiKey';
import { getDatabaseChatEntryList } from '../getDatabaseChatEntryList';

// Type returned by commands from Rust side
interface ChatResponse {
  response: string;
  created_at: string;
}

export default defineComponent({
  name: 'App',
  components: {
    ChatSessions,
    ChatHistory,
    ChatInputForm
  },
  setup() {
    const router = useRouter();

    // ---- State management -----------------------------------------------------
    const input = ref('');
    const currentSessionId = ref<SessionId | null>(null);
    const selectedModel = ref<ModelName | null>(null);
    const apiKeyInput = ref('');
    const isApiKeySet = ref(false);
    const showDialog = ref(true);
    const lastAnswerReceivedTime = ref<dayjs.Dayjs | null>(null);
    const partialAnswer = ref('');
    const lastUserQuestion = ref('');

    // ---- Functions: either pure or with side effects --------------------------
    /**
     * Scroll the chat view to the bottom
     */
    const scrollToBottom = () => {
      const chatHistoryElement = document.querySelector('#app header');
      if (chatHistoryElement) {
        chatHistoryElement.scrollTop = chatHistoryElement.scrollHeight;
      }
    };

    /**
     * Get the latest chat history and set currentSessionId to the newest session
     */
    const loadChatHistory = async () => {
      const history = await getDatabaseChatEntryList();
      if (!history || history.length === 0) return;

      // Reverse history to get the newest session at the front
      const reversedHistory = [...history].reverse();
      currentSessionId.value = reversedHistory[0].session_id;

      // Scroll after rendering
      nextTick(() => {
        scrollToBottom();
      });
    };

    /**
     * Navigate to Settings page
     */
    const goToSettings = () => {
      router.push('/settings');
    };

    /**
     * Handler for when the user sends a message
     */
    const handleSubmit = async (userInput: string, encodedImageList?: EncodedImage[]) => {
      // Validation
      if (!currentSessionId.value || !userInput.trim()) return;

      // Do not proceed if API key is not found
      const api_key = await getApiKey();
      if (!api_key) return;

      // Initialize streaming variables
      lastUserQuestion.value = userInput;
      partialAnswer.value = '';

      // Subscribe to the "token" event to receive partial responses
      const unlisten = await listen<string>('token', (event) => {
        partialAnswer.value += event.payload;
      });

      try {
        // Call the Rust-side command to start streaming
        const finalResponse = (await invoke('stream_chatgpt_response', {
          inputSessionId: currentSessionId.value,
          message: userInput,
          model: selectedModel.value,
          apiKey: api_key,
          base64Images: encodedImageList
        })) as ChatResponse;

        // Record the time when the final response is received after all chunks
        lastAnswerReceivedTime.value = dayjs(finalResponse.created_at);
      } catch (error) {
        console.error('Error streaming:', error);
      } finally {
        // Unsubscribe from the event after streaming is done
        unlisten();
        // Clear the partial text after generation is complete
        partialAnswer.value = '';
      }

      // Reload the latest chat history and update the display
      await loadChatHistory();
      nextTick(() => {
        scrollToBottom();
      });
    };

    // ---- Lifecycle hooks ------------------------------------------------------
    onMounted(async () => {
      // Check for API key on app startup
      const key = await getApiKey();
      if (!key) {
        isApiKeySet.value = false;
        router.push('/settings');
      } else {
        isApiKeySet.value = true;
        showDialog.value = false;
        // Load chat sessions
        await loadChatHistory();
      }
    });

    // ---- Return elements used in the template ---------------------------------
    return {
      input,
      currentSessionId,
      selectedModel,
      apiKeyInput,
      isApiKeySet,
      showDialog,
      lastAnswerReceivedTime,
      partialAnswer,
      lastUserQuestion,
      goToSettings,
      handleSubmit
    };
  }
});
</script>

<style scoped></style>
