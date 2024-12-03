<template>
    <aside id="chat-sessions" class="d-flex flex-column bg-light p-3">
        <ul class="list-group list-group-flush flex-grow-1 overflow-auto mb-3">
            <li v-for="(_, sessionId) in rawChats" :key="sessionId" 
                :class="['list-group-item', 'cursor-pointer', { active: currentSessionId === sessionId }]"
                @click="loadSession(sessionId)">
                Chat Session: {{ sessionId }}
            </li>
        </ul>
        <button class="btn btn-outline-secondary mb-3" @click="$emit('new-session')">New Session</button>
        <ModelSelector v-if="isApiKeySet" :isApiKeySet="isApiKeySet" :selectedModel="selectedModel"
            @update:selectedModel="handleModelChange" />
    </aside>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import ModelSelector from './ModelSelector.vue';
import { ModelName, RawChats, SessionId } from '../types';

export default defineComponent({
    components: { ModelSelector },
    props: {
        rawChats: Object as () => RawChats,
        currentSessionId: {
            type: [String, null] as PropType<SessionId | null>,
            default: null
        },
        isApiKeySet: Boolean,
        selectedModel: {
            type: [String, null] as PropType<ModelName | null>,
            default: null
        },
    },
    methods: {
        loadSession(sessionId: SessionId) {
            this.$emit('session-changed', sessionId);
        },
        handleModelChange(newVal: ModelName) {
            this.$emit('update:selectedModel', newVal)
        }
    }
});
</script>

<style scoped>
aside#chat-sessions {
    width: 25%;
    max-width: 300px;
    min-width: 200px;
}

.list-group-item {
    cursor: pointer;
    transition: background-color 0.3s ease;
}

.list-group-item.active {
    background-color: #007bff;
    color: #fff;
    font-weight: bold;
}

.list-group-item:hover {
    background-color: #e0e0e0; /* Change to a slightly darker shade for better contrast */
    color: #333; /* Ensure text color remains readable */
}

.btn {
    width: 100%;
}
</style>

