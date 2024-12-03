<template>
  <div id="app" class="d-flex vh-100">
    <aside class="col-3 overflow-auto border-end">
      <ChatSessions v-model:currentSessionId="currentSessionId" :isApiKeySet="isApiKeySet"
        v-model:selectedModel="selectedModel" />
    </aside>
    <main class="col-9 d-flex flex-column p-3">
      <ApiKeyDialog v-if="showDialog" @api-key-set="onApiKeySaved" />
      <header class="flex-grow-1 overflow-auto mb-3">
        <ChatHistory :currentSessionId="currentSessionId" :lastAnswerReceivedTime="lastAnswerReceivedTime" />
      </header>
      <footer class="mt-auto">
        <ChatInputForm :onSubmit="handleSubmit" />
      </footer>
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ApiKeyDialog from './components/ApiKeyDialog.vue';
import ChatSessions from './components/ChatSessions.vue';
import ChatHistory from './components/ChatHistory.vue';
import ChatInputForm from './components/ChatInputForm.vue';
import { getApiKey } from './getApiKey';
import { getDatabaseChatEntryList } from './getDatabaseChatEntryList';
import { getChatGptResponse } from './getChatGptResponse';
import { SessionId, UserInput, ModelName, ApiKey } from './types';
import { EncodedImage } from './types';
import dayjs from 'dayjs';

interface ComponentData {
  input: string;
  currentSessionId: SessionId | null;
  selectedModel: ModelName | null;
  apiKeyInput: string;
  isApiKeySet: boolean;
  showDialog: boolean;
  lastAnswerReceivedTime: dayjs.Dayjs | null;
}

export default defineComponent({
  components: {
    ChatSessions,
    ApiKeyDialog,
    ChatHistory,
    ChatInputForm,
  },
  data(): ComponentData {
    return {
      input: '',
      currentSessionId: null,
      selectedModel: null,
      apiKeyInput: '',
      isApiKeySet: false,
      showDialog: true,
      lastAnswerReceivedTime: null,
    };
  },

  computed: {
  },

  async mounted() {
    const key = await getApiKey();

    if (!key) {
      this.isApiKeySet = false;
      return;
    } else {
      this.isApiKeySet = true;
      this.showDialog = false;
      await this.loadChatHistory();
    }
  },

  methods: {
    async handleSubmit(input: string, EncodedImageList?: EncodedImage[]) {
      if (input.trim() === '') return;

      const api_key = await getApiKey();

      if (!api_key) return;
      if (!this.currentSessionId) return;

      const userInput = input as UserInput;
      const selectedModel = this.selectedModel as ModelName;

      const res = await getChatGptResponse(
        this.currentSessionId,
        userInput,
        selectedModel,
        api_key as ApiKey,
        EncodedImageList
      );
      if (!res) return;

      this.lastAnswerReceivedTime = res.created_at

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    async loadChatHistory() {
      const history = await getDatabaseChatEntryList();

      if (!history) return;
      if (history.length == 0) return;

      const reversedHistory = history.reverse();
      this.currentSessionId = reversedHistory[0].session_id;

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    scrollToBottom() {
      const chatHistoryElement = this.$el.querySelector('header');
      if (chatHistoryElement) {
        chatHistoryElement.scrollTop = chatHistoryElement.scrollHeight;
      }
    },

    async onApiKeySaved() {
      this.showDialog = false;
      this.isApiKeySet = true;
      await this.loadChatHistory();
    },
  },
});
</script>

<style scoped></style>
