<template>
  <div id="app">
    <aside id="chat-sessions">
      <ul>
        <li v-for="(_, sessionId) in rawChats" :key="sessionId" :class="{ active: currentSessionId === sessionId }"
          @click="loadSession(sessionId)">
          Chat Session {{ sessionId }}
        </li>
      </ul>
      <button @click="createNewSession">New Session</button>
      <ModelSelector v-if="isApiKeySet" :isApiKeySet="isApiKeySet" v-model:selectedModel="selectedModel" />
    </aside>
    <main>
      <ApiKeyDialog v-if="showDialog" @api-key-set="onApiKeySaved" />
      <header>
        <div id="chat-history">
          <div v-for="(entry, index) in chatHistory" :key="index" class="chat-entry">
            <div class="user-message">
              <strong>You:</strong>
              <pre>{{ entry.question }}</pre>
            </div>
            <div class="gpt-response" v-html="entry.answer"></div>
          </div>
        </div>
        <div>
          <form @submit.prevent="handleSubmit" class="input-form">
            <textarea v-model="input" placeholder="Ask ChatGPT..." rows="4" cols="50"
              @keydown="checkCtrlEnter"></textarea>
            <button type="submit">Send</button>
          </form>
        </div>
      </header>
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ApiKeyDialog from './components/ApiKeyDialog.vue';
import { getApiKey } from './getApiKey';
import { getChatHistory } from './getChatHistory';
import { getChatGptResponse } from './getChatGptResponse';
import { generateSessionId } from './generateSessionId';
import ModelSelector from './components/ModelSelector.vue';
import { renderMarkdown } from './renderMarkdown';
import { getRawChatsFromDatabaseChatEntries } from './getRawChatsFromDatabaseChatEntries';
import { SessionId, UserInput, ModelName, ApiKey, HtmlChatEntry, RawChats } from './types';

interface ComponentData {
  input: string;
  rawChats: RawChats;
  currentSessionId: SessionId | null;
  selectedModel: string;
  apiKeyInput: string;
  isApiKeySet: boolean;
  showDialog: boolean;
}

export default defineComponent({
  components: {
    ModelSelector,
    ApiKeyDialog,
  },
  data(): ComponentData {
    return {
      input: '',
      rawChats: {},
      currentSessionId: null,
      selectedModel: '',
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

  watch: {},

  methods: {
    async handleSubmit() {
      if (this.input.trim() === '') return;
      const api_key = await getApiKey();

      if (!api_key) return;
      if (!this.currentSessionId) return;

      const input = this.input as UserInput;
      const selectedModel = this.selectedModel as ModelName;

      const res = await getChatGptResponse(
        this.currentSessionId,
        input,
        selectedModel,
        api_key as ApiKey,
      );
      if (!res) return;

      if (!this.rawChats[this.currentSessionId]) {
        this.rawChats[this.currentSessionId] = [];
      }

      this.rawChats[this.currentSessionId].push({ question: input, answer: res });
      this.input = '';

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    async loadChatHistory() {
      const history = await getChatHistory();
      if (!history) return;
      if (history.length == 0) return;

      this.rawChats = getRawChatsFromDatabaseChatEntries(history);

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },
    checkCtrlEnter(event: KeyboardEvent) {
      if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
        this.handleSubmit();
      }
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

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
}

aside#chat-sessions {
  width: 25%;
  background-color: #f0f0f0;
  padding: 10px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}

#chat-sessions ul {
  list-style: none;
  padding: 0;
  margin: 0;
  flex-grow: 1;
  overflow-y: auto;
}

#chat-sessions li {
  padding: 10px;
  cursor: pointer;
  border-bottom: 1px solid #ccc;
}

#chat-sessions li.active {
  background-color: #dcdcdc;
  font-weight: bold;
}

#chat-sessions button {
  margin-top: 10px;
  padding: 10px;
  border: none;
  background-color: #e0e0e0;
  cursor: pointer;
}

#chat-sessions button:hover {
  background-color: #d0d0d0;
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

#chat-history {
  flex-grow: 1;
  overflow-y: auto;
  border-bottom: 1px solid #ccc;
  margin-bottom: 10px;
  padding-right: 10px;
}

.chat-entry {
  margin-bottom: 10px;
}

.user-message {
  margin-bottom: 5px;
}

.input-form {
  display: flex;
  align-items: center;
}

textarea {
  flex-grow: 1;
  margin-right: 10px;
  padding: 10px;
  font-size: 14px;
  border: 1px solid #ccc;
  border-radius: 4px;
  resize: none;
}

button[type="submit"] {
  padding: 10px 20px;
  border: none;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  border-radius: 4px;
}

button[type="submit"]:hover {
  background-color: #0056b3;
}
</style>
