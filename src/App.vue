<template>
  <div id="app">
    <aside id="chat-sessions">
      <ul>
        <li
          v-for="(session, index) in chatSessions"
          :key="index"
          :class="{ active: currentSessionIndex === index }"
          @click="loadSession(index)"
        >
          Chat Session {{ index + 1 }}
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
      chatSessions: [[] as ChatEntry[]],
      currentSessionIndex: 0,
    };
  },
  computed: {
    chatHistory() {
      return this.chatSessions[this.currentSessionIndex];
    }
  },
  mounted() {
    this.loadChatHistory();
  },
  methods: {
    async handleSubmit() {
      if (this.input.trim() === '') return;

      try {
        const res: string = await invoke('chat_gpt', { message: this.input });
        const markdownHtml: string = await this.renderMarkdown(res); // Use await to resolve the promise
        this.chatSessions[this.currentSessionIndex].push({ question: this.input, answer: res, markdownHtml });
        await this.saveChatEntry(this.input, res);
        this.input = ''; // Clear the input after saving
      } catch (error) {
        console.error('Error calling API:', error);
      }
    },
    async loadChatHistory() {
      try {
        const history: Array<{ question: string, answer: string }> = await invoke('get_chat_history');
        console.log(history)
        history.forEach(async (entry) => {
          if (entry.answer == null) {
            return
          }
          const markdownHtml = await this.renderMarkdown(entry.answer); // Use await to resolve the promise
          this.chatSessions[this.currentSessionIndex].push({
            question: entry.question,
            answer: entry.answer,
            markdownHtml
          });
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
    loadSession(index: number) {
      this.currentSessionIndex = index;
    },
    createNewSession() {
      this.chatSessions.push([]);
      this.currentSessionIndex = this.chatSessions.length - 1;
    },
    async saveChatEntry(question: string, answer: string) {
      // This function can be used to save the chat entry to the database or perform any additional processing
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
  width: calc(100% - 240px); /* 左右に20pxずつの余白を確保 */
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
