<template>
    <div id="chat-history" class="overflow-auto border-bottom mb-3 pr-3">
        <!-- Past history (User's questions + GPT's answers) -->
        <div v-for="(entry, index) in sortedChatHistory" :key="index" class="chat-entry mb-3">
            <div class="user-message mb-1">
                <strong>You:</strong>
                <pre class="bg-light p-2 rounded">{{ entry.question }}</pre>
            </div>
            <div class="gpt-response bg-secondary text-white p-2 rounded" v-html="entry.answer"></div>
        </div>

        <!-- Temporary display of the ongoing (streaming) answer -->
        <div v-if="streamingAnswer.trim() !== ''" class="chat-entry mb-3">
            <!-- Display the user's question at the top -->
            <div class="user-message mb-1">
                <strong>You:</strong>
                <pre class="bg-light p-2 rounded">{{ lastUserQuestion }}</pre>
            </div>

            <div class="gpt-response bg-secondary text-white p-2 rounded">
                <!-- Insert Markdown-rendered HTML via v-html -->
                <div v-html="partialAnswerHtml"></div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType, ref, computed, watch } from 'vue';
import dayjs from 'dayjs';

// Import types and utilities
import { SessionId, DatabaseChatEntry, Markdown } from '../types';
import { getDatabaseChatEntryBySession } from '../getDatabaseChatEntryBySession';
import { renderMarkdown } from '../renderMarkdown';

export default defineComponent({
    name: 'ChatHistory',
    props: {
        currentSessionId: {
            type: [String, null] as PropType<SessionId | null>,
            default: null
        },
        lastAnswerReceivedTime: {
            type: Object as () => dayjs.Dayjs | null,
            default: null
        },
        streamingAnswer: {
            type: String,
            default: ''
        },
        lastUserQuestion: {
            type: String,
            default: ''
        }
    },
    setup(props) {
        // ----------------------------------
        // State definitions (ref)
        // ----------------------------------
        const databaseChatEntryBySession = ref<DatabaseChatEntry[]>([]);

        // ----------------------------------
        // Side effects (watch)
        // ----------------------------------
        // When the session ID changes, re-fetch history from the DB
        watch(
            () => props.currentSessionId,
            async (newVal) => {
                if (newVal) {
                    await updateChatHistory(newVal);
                }
            }
        );

        // Also re-fetch history when a new answer is received
        watch(
            () => props.lastAnswerReceivedTime,
            async () => {
                if (props.currentSessionId) {
                    await updateChatHistory(props.currentSessionId);
                }
            }
        );

        // ----------------------------------
        // Computed properties
        // ----------------------------------
        const sortedChatHistory = computed(() => {
            // Sort received history by created_at in ascending order
            if (!databaseChatEntryBySession.value) return [];
            return [...databaseChatEntryBySession.value].sort((a, b) =>
                a.created_at.diff(b.created_at)
            );
        });

        // Convert the streaming text to HTML
        const partialAnswerHtml = computed(() => {
            if (!props.streamingAnswer.trim()) return '';
            return renderMarkdown(props.streamingAnswer as Markdown);
        });

        // ----------------------------------
        // Methods
        // ----------------------------------
        async function updateChatHistory(sessionId: SessionId) {
            const dbEntries = await getDatabaseChatEntryBySession(sessionId);
            if (dbEntries) {
                databaseChatEntryBySession.value = dbEntries;
            }
        }

        // ----------------------------------
        // Expose to template
        // ----------------------------------
        return {
            // State
            databaseChatEntryBySession,

            // Computed
            sortedChatHistory,
            partialAnswerHtml
        };
    }
});
</script>

<style scoped></style>
