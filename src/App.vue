<template>
  <div id="app">
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
      chatHistory: [] as ChatEntry[],
    };
  },
  methods: {
    async handleSubmit() {
      if (this.input.trim() === '') return;

      try {
        const res: string = await invoke('chat_gpt', { message: this.input });
        const markdownHtml: string = await this.renderMarkdown(res);
        this.chatHistory.push({ question: this.input, answer: res, markdownHtml });
        this.input = ''; // Clear the input after sending
      } catch (error) {
        console.error('Error calling API:', error);
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
  },
});
</script>

<style scoped>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
  position: relative;
  min-height: 100vh;
  padding-bottom: 120px; /* Height of the fixed form */
  box-sizing: border-box;
}

textarea {
  width: 100%;
  max-width: 600px;
  height: 100px;
  margin-bottom: 10px;
  padding: 10px;
  font-size: 16px;
  font-family: inherit;
  box-sizing: border-box;
}

#chat-history {
  margin-bottom: 20px;
  text-align: left;
  overflow-y: auto;
  max-height: calc(100vh - 180px); /* Adjust max-height for scrolling */
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
  left: 0;
  width: 100%;
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
