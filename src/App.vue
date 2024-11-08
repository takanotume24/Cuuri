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
      <div class="model-selector">
        <label for="model-select">Select Model:</label>
        <select id="model-select" v-model="selectedModel">
          <option v-for="model in availableModels" :key="model" :value="model">
            {{ model }}
          </option>
        </select>
      </div>
    </aside>
    <main>
      <header>
        <div id="chat-history">
          <div v-for="(entry, index) in chatHistory" :key="index" class="chat-entry">
            <div class="user-message"><strong>You:</strong> {{ entry.question }}</div>
            <div class="gpt-response" v-html="entry.markdownHtml"></div>
          </div>
        </div>
        <div v-if="!apiKeySet" class="modal-overlay">
          <div class="modal-dialog">
            <h2>Enter your OpenAI API Key</h2>
            <input v-model="apiKeyInput" placeholder="Enter your OpenAI API key" />
            <button @click="saveApiKey">Save API Key</button>
          </div>
        </div>
        <div v-else>
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
import { invoke } from '@tauri-apps/api/core';
import { marked } from 'marked';

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
      availableModels: [] as string[],
      apiKeyInput: '',
      apiKeySet: false,
    };
  },
  computed: {
    chatHistory() {
      return this.chatSessions[this.currentSessionId] || [];
    }
  },
  async mounted() {
    const key = await this.getApiKey();

    if (!key) {
      this.apiKeySet = false;
      return;
    } else {
      this.apiKeySet = true;
      await this.loadChatHistory();
      await this.fetchAvailableModels();
      if (Object.keys(this.chatSessions).length === 0) {
        await this.createNewSession();
      }
    }
  },
  watch: {
  },
  methods: {
    async getApiKey(): Promise<string | null> {
      try {
        return await invoke<string>('get_openai_api_key');
      } catch (error) {
        console.error('Failed to check API key:', error);
        return null;
      }
    },
    async getAvailableModels(
      apiKey: string,
    ): Promise<string[] | null> {
      try {
        return await invoke<string[]>('get_available_models', {
          apiKey: apiKey,
        });
      } catch (error) {
        console.error('Failed to get Available models:', error);
        return null;
      }
    },
    async getDefaultModel(): Promise<string | null> {
      try {
        return await invoke<string>('get_default_model');
      } catch (error) {
        console.error('Failed to get default model:', error);
        return null;
      }
    },
    async generateSessionId(): Promise<string | null> {
      try {
        return await invoke<string>('generate_session_id');
      } catch (error) {
        console.error('Failed to generate session id:', error);
        return null;
      }
    },
    async getChatGptResponse(
      currentSessionId: string,
      input: string,
      selectedModel: string,
      apiKey: string,
    ): Promise<string | null> {
      try {
        return await invoke('chat_gpt', {
          inputSessionId: currentSessionId,
          message: input,
          model: selectedModel,
          apiKey: apiKey,
        });;
      } catch (error) {
        console.error('Failed to get the response from chatgpt api:', error);
        return null;
      }
    },
    async getChatHistory(): Promise<Array<{ session_id: string, question: string, answer: string }> | null> {
      try {
        return await invoke('get_chat_history');
      } catch (error) {
        console.error('Failed to get chat history:', error);
        return null;
      }
    },
    async fetchAvailableModels() {
      const apiKey = await this.getApiKey();

      if (!apiKey) {
        return;
      };

      const models = await this.getAvailableModels(apiKey);

      if (!models) {
        return;
      }

      this.availableModels = models;
      if (models.length > 0) {
        const defaultModel = await this.getDefaultModel();

        if (!defaultModel) {
          this.selectedModel = '';
          return;
        }

        this.selectedModel = models.includes(defaultModel) ? defaultModel : '';
      }

    },

    async handleSubmit() {
      if (this.input.trim() === '') return;

      const api_key = await this.getApiKey();

      if (!api_key) {
        return;
      }

      const res = await this.getChatGptResponse(
        this.currentSessionId,
        this.input,
        this.selectedModel,
        api_key,
      )

      if (!res) {
        return
      }

      const markdownHtml: string = await this.renderMarkdown(res);

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
      const history = await this.getChatHistory();

      if (!history) {
        return;
      }
      for (const entry of history) {
        if (entry.answer == null) {
          continue;
        }
        const markdownHtml = await this.renderMarkdown(entry.answer);
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
    async renderMarkdown(markdownText: string): Promise<string> {
      return Promise.resolve(marked(markdownText));
    },
    loadSession(sessionId: string) {
      this.currentSessionId = sessionId;
      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },
    async createNewSession() {
      const newSessionId = await this.generateSessionId();

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
    async saveApiKey() {
      try {
        await invoke('set_openai_api_key', { apiKey: this.apiKeyInput });
        this.apiKeySet = true;
        await this.loadChatHistory();
        await this.fetchAvailableModels();
        if (Object.keys(this.chatSessions).length === 0) {
          await this.createNewSession();
        }
      } catch (error) {
        console.error('Failed to save API key:', error);
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

.modal-dialog {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  text-align: center;
}

.modal-dialog input {
  width: 80%;
  padding: 10px;
  margin-bottom: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.modal-dialog button {
  padding: 10px 20px;
  border: none;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  border-radius: 4px;
}

.modal-dialog button:hover {
  background-color: #0056b3;
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

.model-selector {
  margin-top: auto;
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
