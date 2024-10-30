<template>
  <div id="app">
    <header>
      <form @submit.prevent="handleSubmit">
        <input
          v-model="input"
          type="text"
          placeholder="Ask ChatGPT..."
        />
        <button type="submit">Send</button>
      </form>
      <div id="response">{{ response }}</div>
    </header>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";

export default {
  data() {
    return {
      input: '',
      response: '',
    };
  },
  methods: {
    async handleSubmit() {
      try {
        const res = await invoke('chat_gpt', { message: this.input });
        this.response = res;
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
</style>
