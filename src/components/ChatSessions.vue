<template>
    <aside id="chat-sessions" class="d-flex flex-column bg-light p-3">
        <NewSessionButton @new-session="createNewSession" />
            <ModelSelector v -if=" isApiKeySet" :isApiKeySet="isApiKeySet" :selectedModel="selectedModel"
            @update:selectedModel="handleModelChange" class="mb-3" />
        <ul class="list-group list-group-flush flex-grow-1 overflow-auto mb-3">
            <li v-for="sessionId in sessionIdList" :key="sessionId"
                :class="['list-group-item', 'cursor-pointer', { active: currentSessionId === sessionId }]"
                @click="selectSession(sessionId)">
                Chat Session: {{ sessionId }}
            </li>
        </ul>
    </aside>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import ModelSelector from './ModelSelector.vue';
import NewSessionButton from './NewSessionButton.vue';
import { ModelName, SessionId } from '../types';
import { getSessionIdList } from '../getSessionIdList';
import { generateSessionId } from '../generateSessionId';

export default defineComponent({
    components: { ModelSelector, NewSessionButton },
    props: {
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
    data() {
        return {
            localCurrentSessionId: null as SessionId | null,
            sessionIdList: [] as SessionId[],
        };
    },
    watch: {
        currentSessionId(newVal: SessionId, _: SessionId) {
            this.localCurrentSessionId = newVal;
        },
    },
    async mounted() {
        await this.fetchSessionIdList();
    },
    methods: {
        selectSession(sessionId: SessionId) {
            this.localCurrentSessionId = sessionId;
            this.$emit('update:currentSessionId', sessionId);
        },
        handleModelChange(newVal: ModelName) {
            this.$emit('update:selectedModel', newVal);
        },
        async fetchSessionIdList() {
            try {
                // Call your getSessionIdList function
                const sessionIdList = await getSessionIdList();
                if (!sessionIdList) return;
                this.sessionIdList = sessionIdList;
            } catch (error) {
                console.error('Failed to fetch session IDs:', error);
            }
        },
        async createNewSession() {
            const newSessionId = await generateSessionId();
            if (!newSessionId) return;

            this.sessionIdList.push(newSessionId)
            console.log(this.sessionIdList)
            this.$emit('update:currentSessionId', newSessionId);
        },
    },
});
</script>

<style scoped></style>
