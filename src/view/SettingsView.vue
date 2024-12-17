<template>
    <div class="container mt-5">
        <div class="card">
            <div class="card-header">
                <h2>Settings</h2>
            </div>
            <div class="card-body">
                <div class="form-group">
                    <label for="apiKeyInput">API Key</label>
                    <input id="apiKeyInput" v-model="apiKeyInput" class="form-control" :placeholder="placeholderText" />
                </div>
                <div class="d-flex justify-content-between mt-3">
                    <button @click="saveApiKey" class="btn btn-primary">Save API Key</button>
                    <button @click="backToHomeView" class="btn btn-secondary">Close</button>
                </div>
                <p v-if="message" class="mt-3 alert"
                    :class="{ 'alert-success': message.includes('successfully'), 'alert-danger': message.includes('Failed') }">
                    {{ message }}
                </p>
            </div>
        </div>
    </div>
</template>


<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { getApiKey } from '../getApiKey';

export default {
    data() {
        return {
            apiKeyInput: '',
            placeholderText: 'Enter your OpenAI API key',
            apiKeySet: false,
            message: '',
        };
    },
    methods: {
        async saveApiKey() {
            try {
                await invoke('set_openai_api_key', { apiKey: this.apiKeyInput });
                this.message = 'API key saved successfully!';
            } catch (error) {
                this.message = 'Failed to save API key: ' + error;
                console.error('Failed to save API key:', error);
            }
        },
        backToHomeView() {
            this.$router.push('/');
        },
    },
    async created() {
        try {
            const apiKey = await getApiKey();
            if (apiKey) {
                this.apiKeyInput = apiKey;
            }
        } catch (error) {
            console.error('Failed to retrieve API key:', error);
        }
    },
};
</script>

<style></style>