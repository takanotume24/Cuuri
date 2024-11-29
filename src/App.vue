<template>
  <div id="app">
    <aside id="chat-sessions">
      <ul>
        <li v-for="(_, sessionId) in chatSessions" :key="sessionId" :class="{ active: currentSessionId === sessionId }"
          @click="loadSession(sessionId)">
          Chat Session {{ sessionId }}
        </li>
      </ul>
      <button @click="createNewSession">New Session</button>
      <ModelSelector v-if="isApiKeySet" :isApiKeySet="isApiKeySet" />
    </aside>
    <main>
      <ApiKeyDialog v-if="showDialog" @api-key-set="onApiKeySaved" />
      <header>
        <div id="chat-history">
          <div v-for="(entry, index) in chatHistory" :key="index" class="chat-entry">
            <div class="user-message"><strong>You:</strong> {{ entry.question }}</div>
            <div class="gpt-response" v-html="entry.markdownHtml"></div>
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

interface ChatEntry {
  question: string;
  answer: string;
  markdownHtml: string;
}

export default defineComponent({
  data() {
    return {
      input: '',
      chatSessions: {} as Record<string, ChatEntry[]>,
      currentSessionId: '',
      selectedModel: '',
      apiKeyInput: '',
      isApiKeySet: false,
      showDialog: true,
    };
  },

  computed: {
    chatHistory() {
      return this.chatSessions[this.currentSessionId] || [];
    }
  },

  async mounted() {
    const key = await getApiKey();

    if (!key) {
      this.isApiKeySet = false;
      return;
    } else {
      this.isApiKeySet = true;
      await this.loadChatHistory();
      if (Object.keys(this.chatSessions).length === 0) {
        await this.createNewSession();
      }
    }
  },

  watch: {
  },

  methods: {
    async handleSubmit() {
      if (this.input.trim() === '') return;
      const api_key = await getApiKey();

      if (!api_key) {
        return;
      }

      const res = await getChatGptResponse(
        this.currentSessionId,
        this.input,
        this.selectedModel,
        api_key,
      )

      if (!res) {
        return
      }

      const markdownHtml: string = await renderMarkdown(res);

      if (!this.chatSessions[this.currentSessionId]) {
        this.chatSessions[this.currentSessionId] = [];
      }

      this.chatSessions[this.currentSessionId].push({ question: this.input, answer: res, markdownHtml });
      this.input = '';

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    async loadChatHistory() {
      const history = await getChatHistory();

      if (!history) {
        return;
      }
      for (const entry of history) {
        if (entry.answer == null) {
          continue;
        }
        const markdownHtml = await renderMarkdown(entry.answer);
        if (!this.chatSessions[entry.session_id]) {
          this.chatSessions[entry.session_id] = [];
        }
        this.chatSessions[entry.session_id].push({
          question: entry.question,
          answer: entry.answer,
          markdownHtml
        });
      }

      if (!this.currentSessionId && Object.keys(this.chatSessions).length > 0) {
        this.currentSessionId = Object.keys(this.chatSessions)[0];
      }
      this.$nextTick(() => {
        this.scrollToBottom();
      });

    },
    checkCtrlEnter(event: KeyboardEvent) {
      if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
        this.handleSubmit();
      }
    },

    loadSession(sessionId: string) {
      this.currentSessionId = sessionId;
      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    async createNewSession() {
      const newSessionId = await generateSessionId();

      if (!newSessionId) {
        return;
      }

      this.chatSessions[newSessionId] = [];
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
      if (Object.keys(this.chatSessions).length === 0) {
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
  font-weight: bold;
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
