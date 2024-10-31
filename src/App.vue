<template>
  <div id="app">
    <aside id="chat-sessions">
      <ul>
        <li
          v-for="(session, sessionId) in chatSessions"
          :key="sessionId"
          :class="{ active: currentSessionId === sessionId }"
          @click="loadSession(sessionId)"
        >
          Chat Session {{ sessionId }}
        </li>
      </ul>
      <button @click="createNewSession">New Session</button>
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
          <textarea
            v-model="input"
            placeholder="Ask ChatGPT..."
            rows="4"
            cols="50"
            @keydown="checkCtrlEnter"
          ></textarea>
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
      chatSessions: {} as Record<string, ChatEntry[]>, // Using a record to map session IDs to chat entries
      currentSessionId: '',
    };
  },
  computed: {
    chatHistory() {
      return this.chatSessions[this.currentSessionId] || [];
    }
  },
  mounted() {
    this.loadChatHistory();
  },
  methods: {
    async handleSubmit() {
      if (this.input.trim() === '') return;

      try {
        const res: string = await invoke('chat_gpt', { inputSessionId: this.currentSessionId, message: this.input });
        const markdownHtml: string = await this.renderMarkdown(res);
        this.chatSessions[this.currentSessionId].push({ question: this.input, answer: res, markdownHtml });
        this.input = ''; // Clear the input after saving
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

        // Set the first session ID as the active session
        this.currentSessionId = Object.keys(this.chatSessions)[0] || '';
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
      const newSessionId: string = await invoke('generate_session_id');
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
    }
  },
});
</script>

<style scoped>
#app {
  display: flex;
}

aside#chat-sessions {
  width: 200px;
  background-color: #e0e0e0;
  border-right: 1px solid #ddd;
  padding: 10px;
  box-sizing: border-box;
  position: relative;
  height: 100vh;
}

aside#chat-sessions ul {
  list-style-type: none;
  padding: 60px 0 0;
  margin: 0;
}

aside#chat-sessions li {
  padding: 10px;
  cursor: pointer;
}

aside#chat-sessions li.active {
  background-color: #dcdcdc;
  font-weight: bold;
}

aside#chat-sessions button {
  position: absolute;
  top: 10px;
  left: 10px;
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
}

main {
  flex: 1;
  padding: 20px;
}

#chat-history {
  margin-bottom: 20px;
  text-align: left;
  overflow-y: auto;
  max-height: calc(100vh - 180px);
}

.chat-entry {
  margin-bottom: 15px;
}

.user-message {
  margin: 5px 0;
}

.gpt-response {
  margin: 5px 0;
  border-left: 2px solid #ddd;
  padding-left: 10px;
}

.input-form {
  position: fixed;
  bottom: 0;
  left: 220px;
  width: calc(100% - 240px); /* Adjust the width for padding */
  background-color: #fff;
  padding: 10px;
  box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.1);
  display: flex;
  justify-content: center;
}

button {
  margin-left: 10px;
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
}
</style>
