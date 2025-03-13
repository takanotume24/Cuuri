<template>
    <div id="chat-history" class="overflow-auto border-bottom mb-3 pr-3">
        <!-- 過去の履歴(ユーザの質問 + GPTの回答) -->
        <div v-for="(entry, index) in sortedChatHistory" :key="index" class="chat-entry mb-3">
            <div class="user-message mb-1">
                <strong>You:</strong>
                <pre class="bg-light p-2 rounded">{{ entry.question }}</pre>
            </div>
            <div class="gpt-response bg-secondary text-white p-2 rounded" v-html="entry.answer"></div>
        </div>

        <!-- ★ 新たに追加: ストリーミング中の回答を一時表示する箇所 -->
        <div v-if="streamingAnswer.trim() !== ''" class="chat-entry mb-3">
            <div class="user-message mb-1">
                <strong>ChatGPT (generating...):</strong>
                <div class="bg-secondary text-white p-2 rounded">
                    {{ streamingAnswer }}
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { SessionId } from '../types';
import { DatabaseChatEntry } from '../types';
import dayjs from 'dayjs';

// DBから履歴を取得する関数
import { getDatabaseChatEntryBySession } from '../getDatabaseChatEntryBySession';

export default defineComponent({
    name: 'ChatHistory',
    props: {
        currentSessionId: {
            type: [String, null] as PropType<SessionId | null>,
            default: null
        },
        lastAnswerReceivedTime: {
            type: [Object, null] as PropType<dayjs.Dayjs | null>,
            default: null
        },
        /**
         * ★ 新規追加:
         * 親コンポーネントがストリーミング中に得た部分的な回答を
         * リアルタイム表示するためのプロパティ
         */
        streamingAnswer: {
            type: String,
            default: ''
        }
    },
    data() {
        return {
            databaseChatEntryBySession: [] as DatabaseChatEntry[]
        };
    },
    watch: {
        // セッションIDが変わったときに履歴を更新
        async currentSessionId(newVal: SessionId, _: SessionId) {
            this.updateChatHistory(newVal);
        },
        // 新しい回答を受信したタイミングで履歴を再取得
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
        async updateChatHistory(sessionId: SessionId) {
            // DBから指定セッションのチャット履歴を取得
            const dbEntries = await getDatabaseChatEntryBySession(sessionId);
            if (!dbEntries) return;
            this.databaseChatEntryBySession = dbEntries;
        }
    },
});
</script>

<style scoped>
/* 必要に応じてスタイルを追加 */
</style>