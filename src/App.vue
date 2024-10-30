<template>
  <div id="app">
    <header>
      <form @submit.prevent="handleSubmit">
        <textarea
          v-model="input"
          placeholder="Ask ChatGPT..."
          rows="4"
          cols="50"
        ></textarea>
        <button type="submit">Send</button>
      </form>
      <div id="chat-history">
        <div v-for="(entry, index) in chatHistory" :key="index" class="chat-entry">
          <div class="user-message"><strong>You:</strong> {{ entry.question }}</div>
          <div class="gpt-response"><strong>GPT:</strong> {{ entry.answer }}</div>
        </div>
      </div>
    </header>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";

export default {
  data() {
    return {
      input: '',
      chatHistory: [],
    };
  },
  methods: {
    async handleSubmit() {
      if (this.input.trim() === '') return;

      try {
        const res = await invoke('chat_gpt', { message: this.input });
        this.chatHistory.push({ question: this.input, answer: res });
        this.input = ''; // Clear the input after sending
      } catch (error) {
        console.error('Error calling API:', error);
      }
    },
  },
};
</script>

<style scoped>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
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
  margin-top: 20px;
  text-align: left;
}

.chat-entry {
  margin-bottom: 15px;
}

.user-message, .gpt-response {
  margin: 5px 0;
}
</style>
