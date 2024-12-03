<template>
  <div id="app">
    <ChatSessions :rawChats="rawChats" :currentSessionId="currentSessionId" :isApiKeySet="isApiKeySet"
      v-model:selectedModel="selectedModel" @new-session="createNewSession" @session-changed="loadSession" />
    <main>
      <ApiKeyDialog v-if="showDialog" @api-key-set="onApiKeySaved" />
      <header>
        <ChatHistory :chatHistory="chatHistory" />
        <ChatInputForm :onSubmit="handleSubmit" />
      </header>
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ApiKeyDialog from './components/ApiKeyDialog.vue';
import ChatSessions from './components/ChatSessions.vue';
import ChatHistory from './components/ChatHIstory.vue';
import ChatInputForm from './components/ChatInputForm.vue';
import { getApiKey } from './getApiKey';
import { getChatHistory } from './getChatHistory';
import { getChatGptResponse } from './getChatGptResponse';
import { generateSessionId } from './generateSessionId';
import { renderMarkdown } from './renderMarkdown';
import { getRawChatsFromDatabaseChatEntries } from './getRawChatsFromDatabaseChatEntries';
import { SessionId, UserInput, ModelName, ApiKey, HtmlChatEntry, RawChats } from './types';
import { EncodedImage } from './types';

interface ComponentData {
  input: string;
  rawChats: RawChats;
  currentSessionId: SessionId | null;
  selectedModel: ModelName | null;
  apiKeyInput: string;
  isApiKeySet: boolean;
  showDialog: boolean;
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
      rawChats: {},
      currentSessionId: null,
      selectedModel: null,
      apiKeyInput: '',
      isApiKeySet: false,
      showDialog: true,
    };
  },

  computed: {
    chatHistory() {
      if (!this.currentSessionId) return [];
      const raw_chat_entry_array = this.rawChats[this.currentSessionId];
      const html_chat_entry_array: Array<HtmlChatEntry> = raw_chat_entry_array.map(x => ({
        question: x.question,
        answer: renderMarkdown(x.answer),
      }));
      return html_chat_entry_array;
    },
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
      if (Object.keys(this.rawChats).length === 0) {
        await this.createNewSession();
      }
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

      if (!this.rawChats[this.currentSessionId]) {
        this.rawChats[this.currentSessionId] = [];
      }

      this.rawChats[this.currentSessionId].push({ question: userInput, answer: res });

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    async loadChatHistory() {
      const history = await getChatHistory();

      if (!history) return;
      if (history.length == 0) return;

      const reversedHistory = history.reverse();
      this.currentSessionId = reversedHistory[0].session_id;
      this.rawChats = getRawChatsFromDatabaseChatEntries(reversedHistory);

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    loadSession(sessionId: SessionId) {
      this.currentSessionId = sessionId;
      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    async createNewSession() {
      const newSessionId = await generateSessionId();
      if (!newSessionId) return;

      this.rawChats[newSessionId] = [];
      this.currentSessionId = newSessionId;
      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    scrollToBottom() {
      const chatHistoryElement = this.$el.querySelector('#chat-history');
      if (chatHistoryElement) {
        chatHistoryElement.scrollTop = chatHistoryElement.scrollHeight;
      }
    },

    async onApiKeySaved() {
      this.showDialog = false;
      this.isApiKeySet = true;
      await this.loadChatHistory();
      if (Object.keys(this.rawChats).length === 0) {
        await this.createNewSession();
      }
    },
  },
});
</script>

<style scoped>
#app {
  display: flex;
  width: 100%;
  height: 100vh;
  font-family: Arial, sans-serif;
}

main {
  width: 75%;
  display: flex;
  flex-direction: column;
  padding: 10px;
  box-sizing: border-box;
}

header {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}
</style>
