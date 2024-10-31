<template>
  <div id="app">
    <aside id="chat-sessions">
      <ul>
        <li v-for="(session, sessionId) in chatSessions" :key="sessionId"
          :class="{ active: currentSessionId === sessionId }" @click="loadSession(sessionId)">
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
        <form @submit.prevent="handleSubmit" class="input-form">
          <textarea v-model="input" placeholder="Ask ChatGPT..." rows="4" cols="50"
            @keydown="checkCtrlEnter"></textarea>
          <button type="submit">Send</button>
        </form>
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
      availableModels: [] as string[], // List of available models
    };
  },
  computed: {
    chatHistory() {
      return this.chatSessions[this.currentSessionId] || [];
    }
  },
  mounted() {
    this.loadChatHistory();
    this.fetchAvailableModels();
    // Create the initial session if no sessions are loaded
    if (Object.keys(this.chatSessions).length === 0) {
      this.createNewSession();
    }
  },
  methods: {
    async fetchAvailableModels() {
      try {
        const models: string[] = await invoke('get_available_models');
        this.availableModels = models;
        if (models.length > 0) {
          const defaultModel: string = await invoke('get_default_model');
          this.selectedModel = models.includes(defaultModel) ? defaultModel : '';
        }
      } catch (error) {
        console.error('Failed to fetch available models:', error);
      }
    },
    async handleSubmit() {
      if (this.input.trim() === '') return;

      try {
        const res: string = await invoke('chat_gpt', {
          inputSessionId: this.currentSessionId,
          message: this.input,
          model: this.selectedModel // Pass the selected model
        });
        const markdownHtml: string = await this.renderMarkdown(res);

        if (!this.chatSessions[this.currentSessionId]) {
          this.chatSessions[this.currentSessionId] = [];
        }

        this.chatSessions[this.currentSessionId].push({ question: this.input, answer: res, markdownHtml });
        this.input = '';

        this.$nextTick(() => {
          this.scrollToBottom();
        });
      } catch (error) {
        console.error('Error calling API:', error);
      }
    },
    async loadChatHistory() {
      try {
        const history: Array<{ session_id: string, question: string, answer: string }> = await invoke('get_chat_history');
        history.forEach(async (entry) => {
          if (entry.answer == null) {
            return;
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
        });

        if (!this.currentSessionId && Object.keys(this.chatSessions).length > 0) {
          this.currentSessionId = Object.keys(this.chatSessions)[0];
        }
        this.$nextTick(() => {
          this.scrollToBottom();
        });
      } catch (error) {
        console.error('Failed to load chat history:', error);
      }
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
      try {
        const newSessionId: string = await invoke('generate_session_id');
        this.chatSessions[newSessionId] = [];
        this.currentSessionId = newSessionId;
        this.$nextTick(() => {
          this.scrollToBottom();
        });
      } catch (error) {
        console.error('Failed to create a new session:', error);
      }
    },
    scrollToBottom() {
      const chatHistoryElement = this.$el.querySelector('#chat-history');
      if (chatHistoryElement) {
        chatHistoryElement.scrollTop = chatHistoryElement.scrollHeight;
      }
    }
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
