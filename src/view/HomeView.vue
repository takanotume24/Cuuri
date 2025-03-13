<template>
  <div id="app" class="d-flex vh-100">
    <!-- 左側: セッション一覧と設定ボタン -->
    <aside class="col-3 overflow-auto border-end">
      <ChatSessions v-model:currentSessionId="currentSessionId" :isApiKeySet="isApiKeySet"
        v-model:selectedModel="selectedModel" />
      <div class="d-flex justify-content-end mt-3">
        <button class="btn btn-primary" @click="goToSettings">Settings</button>
      </div>
    </aside>

    <!-- 右側: チャット履歴 + 入力フォーム -->
    <main class="col-9 d-flex flex-column p-3">
      <header class="flex-grow-1 overflow-auto mb-3">
        <!-- 履歴表示コンポーネント (streamingAnswer 等を渡す) -->
        <ChatHistory :currentSessionId="currentSessionId" :lastAnswerReceivedTime="lastAnswerReceivedTime"
          :streamingAnswer="partialAnswer" :lastUserQuestion="lastUserQuestion" />
      </header>

      <!-- フッターにある入力フォーム -->
      <footer class="mt-auto">
        <ChatInputForm :onSubmit="handleSubmit" />
      </footer>
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { SessionId, ModelName, EncodedImage } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import ChatSessions from '../components/ChatSessions.vue';
import ChatHistory from '../components/ChatHistory.vue';
import ChatInputForm from '../components/ChatInputForm.vue';

import { getApiKey } from '../getApiKey';
import { getDatabaseChatEntryList } from '../getDatabaseChatEntryList';

import dayjs from 'dayjs';

// Rust 側のコマンドが返す型
interface ChatResponse {
  response: string;
  created_at: string;
}

interface ComponentData {
  input: string;
  currentSessionId: SessionId | null;
  selectedModel: ModelName | null;
  apiKeyInput: string;
  isApiKeySet: boolean;
  showDialog: boolean;
  lastAnswerReceivedTime: dayjs.Dayjs | null;
  partialAnswer: string;
  // ★ 追加: 「最後に送信されたユーザー質問」を保持するプロパティ
  lastUserQuestion: string;
}

export default defineComponent({
  name: 'App',
  components: {
    ChatSessions,
    ChatHistory,
    ChatInputForm
  },
  data(): ComponentData {
    return {
      input: '',
      currentSessionId: null,
      selectedModel: null,
      apiKeyInput: '',
      isApiKeySet: false,
      showDialog: true,
      lastAnswerReceivedTime: null,
      partialAnswer: '',
      lastUserQuestion: ''
    };
  },
  async mounted() {
    // アプリ起動時にAPIキーを確認し、未設定なら Settings 画面へ
    const key = await getApiKey();
    if (!key) {
      this.isApiKeySet = false;
      this.$router.push('/settings');
    } else {
      this.isApiKeySet = true;
      this.showDialog = false;
      // セッション履歴を読み込み
      await this.loadChatHistory();
    }
  },
  methods: {
    goToSettings() {
      this.$router.push('/settings');
    },

    /**
     * ユーザーがメッセージを送信したとき
     */
    async handleSubmit(input: string, EncodedImageList?: EncodedImage[]) {
      if (!this.currentSessionId) return;
      if (!input.trim()) return;

      const api_key = await getApiKey();
      if (!api_key) return;

      // ★ 最後に送信されたユーザー質問をセット
      this.lastUserQuestion = input;

      // ストリーミング用変数を初期化
      this.partialAnswer = '';

      // "token" イベントを購読して、部分回答が来るたびに partialAnswer に追記する
      const unlisten = await listen<string>('token', (event) => {
        const tokenChunk = event.payload;
        this.partialAnswer += tokenChunk;
      });

      try {
        // Rust 側コマンドを呼び出してストリーミングを開始
        const finalResponse = (await invoke('stream_chatgpt_response', {
          inputSessionId: this.currentSessionId,
          message: input,
          model: this.selectedModel,
          apiKey: api_key,
          base64Images: EncodedImageList
        })) as ChatResponse;

        // 全チャンク受信が完了すると Rust から最終回答が返る
        this.lastAnswerReceivedTime = dayjs(finalResponse.created_at);
      } catch (error) {
        console.error('Error streaming:', error);
      } finally {
        // ストリーミング終了後にイベント購読を解除
        unlisten();
        // 生成完了後に一時テキストを消す
        this.partialAnswer = '';
      }

      // 履歴を再取得して表示を更新
      await this.loadChatHistory();

      // 下端へスクロール
      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    /**
     * 全体のチャット履歴を取得し、最新セッションを選択する
     */
    async loadChatHistory() {
      const history = await getDatabaseChatEntryList();
      if (!history || history.length === 0) return;

      // 履歴を新しい順にして、先頭(最新)のセッションを選択
      const reversedHistory = history.reverse();
      this.currentSessionId = reversedHistory[0].session_id;

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    /**
     * チャット画面を最下部までスクロール
     */
    scrollToBottom() {
      const chatHistoryElement = this.$el.querySelector('header');
      if (chatHistoryElement) {
        chatHistoryElement.scrollTop = chatHistoryElement.scrollHeight;
      }
    }
  }
});
</script>

<style scoped>
/* 必要に応じてCSSを追加 */
</style>
