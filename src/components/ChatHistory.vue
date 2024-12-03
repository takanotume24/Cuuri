<template>
    <div id="chat-history" class="overflow-auto border-bottom mb-3 pr-3">
        <div v-for="(entry, index) in sortedChatHistory" :key="index" class="chat-entry mb-3">
            <div class="user-message mb-1">
                <strong>You:</strong>
                <pre class="bg-light p-2 rounded">{{ entry.question }}</pre>
            </div>
            <div class="gpt-response bg-secondary text-white p-2 rounded" v-html="entry.answer"></div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { SessionId } from '../types';
import { getDatabaseChatEntryBySession } from '../getDatabaseChatEntryBySession';
import { DatabaseChatEntry } from '../types';
import dayjs from 'dayjs';

export default defineComponent({
    props: {
        currentSessionId: {
            type: [String, null] as PropType<SessionId | null>,
            default: null
        },
        lastAnswerReceivedTime: {
            type: [Object, null] as PropType<dayjs.Dayjs | null>,
            default: null
        },
    },
    data() {
        return {
            databaseChatEntryBySession: [] as DatabaseChatEntry[] // Initialize databaseChatEntryBySession as an empty array
        };
    },
    watch: {
        async currentSessionId(newVal: SessionId, _: SessionId) {
            this.updateChatHistory(newVal);
        },
        lastAnswerReceivedTime(_, __) {
            if (!this.currentSessionId) return;
            this.updateChatHistory(this.currentSessionId);
        }
    },
    computed: {
        sortedChatHistory() {
            if (!this.databaseChatEntryBySession) return [];
            return this.databaseChatEntryBySession.slice().sort((a, b) => {
                return a.created_at.diff(b.created_at);
            });
        }
    },
    methods: {
        async updateChatHistory(session_id: SessionId) {
            const databaseChatEntryBySession = await getDatabaseChatEntryBySession(session_id);
            if (!databaseChatEntryBySession) return;
            this.databaseChatEntryBySession = databaseChatEntryBySession
        }
    },
});
</script>
