<template>
    <aside id="chat-sessions">
        <ul>
            <li v-for="(_, sessionId) in rawChats" :key="sessionId" :class="{ active: currentSessionId === sessionId }"
                @click="loadSession(sessionId)">
                Chat Session {{ sessionId }}
            </li>
        </ul>
        <button @click="createNewSession">New Session</button>
        <ModelSelector v-if="isApiKeySet" :isApiKeySet="isApiKeySet" v-model:selectedModel="selectedModel" />
    </aside>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ModelSelector from './components/ModelSelector.vue';
import { SessionId, RawChats } from '../types';

export default defineComponent({
    components: {
        ModelSelector,
    },
    props: {
        rawChats: Object as () => RawChats,
        currentSessionId: null as SessionId | null,
        isApiKeySet: Boolean,
        selectedModel: String,
    },
    methods: {
        loadSession(sessionId: SessionId) {
            this.$emit('load-session', sessionId);
        },
        createNewSession() {
            this.$emit('create-new-session');
        }
    }
});
</script>

<style scoped>
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
</style>