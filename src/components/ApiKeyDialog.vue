<template>
    <div class="dialog" v-if="isVisible">
        <div class="modal-dialog">
            <h2>Enter your OpenAI API Key</h2>
            <input v-model="apiKeyInput" placeholder="Enter your OpenAI API key" />
            <button @click="saveApiKey">Save API Key</button>
        </div>
    </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core';

export default {
    data() {
        return {
            apiKeyInput: '',
            apiKeySet: false,
            isVisible: true,
        };
    },
    methods: {
        async saveApiKey() {
            try {
                await invoke('set_openai_api_key', { apiKey: this.apiKeyInput });
                this.apiKeySet = true;
                this.isVisible = false;
                this.$emit('api-key-set');
            } catch (error) {
                console.error('Failed to save API key:', error);
            }
        },
    },
    created() {
    },
};
</script>

<style>
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
</style>