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
        <!-- 履歴表示コンポーネント (streamingAnswer を渡す) -->
        <ChatHistory :currentSessionId="currentSessionId" :lastAnswerReceivedTime="lastAnswerReceivedTime"
          :streamingAnswer="partialAnswer" />
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
// Tauri のイベント/Invoke を使用
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

  // ★ ストリーミング中の部分文章を保持する
  partialAnswer: string;
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

      partialAnswer: '' // ストリーミング表示用
    };
  },
  async mounted() {
    // 1) 起動時にAPIキーを確認し、なければ設定画面へ
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
    // 設定画面へ移動
    goToSettings() {
      this.$router.push('/settings');
    },

    /**
     * ユーザーがメッセージを送信したときに呼ばれるメソッド
     */
    async handleSubmit(input: string, EncodedImageList?: EncodedImage[]) {
      if (!this.currentSessionId) return;
      if (!input.trim()) return;

      // APIキー
      const api_key = await getApiKey();
      if (!api_key) return;

      // ストリーミング用変数を初期化
      this.partialAnswer = '';

      // 1) "token" イベントを購読して、トークン(部分回答)が来るたびに partialAnswer に追記
      const unlisten = await listen<string>('token', (event) => {
        const tokenChunk = event.payload;
        this.partialAnswer += tokenChunk;
      });

      try {
        // 2) Rust 側コマンドを呼び出してストリーミングを開始
        const finalResponse = (await invoke('stream_chatgpt_response', {
          inputSessionId: this.currentSessionId,
          message: input,
          model: this.selectedModel,
          apiKey: api_key,
          base64Images: EncodedImageList
        })) as ChatResponse;

        // 3) 全チャンク受信が完了すると Rust から最終回答が返る
        //    -> DBには Rust 側ですでに保存済みなので、改めて履歴を取得
        this.lastAnswerReceivedTime = dayjs(finalResponse.created_at);

      } catch (error) {
        console.error('Error streaming:', error);
      } finally {
        // 4) ストリーミング終了後にイベント購読を解除
        unlisten();
      }

      // 履歴再取得
      await this.loadChatHistory();

      // UIをスクロール
      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    /**
     * セッション履歴を読み込む
     */
    async loadChatHistory() {
      const history = await getDatabaseChatEntryList();
      if (!history || history.length === 0) return;

      // 最新セッションを選択
      const reversedHistory = history.reverse();
      this.currentSessionId = reversedHistory[0].session_id;

      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },

    /**
     * 画面下へスクロール
     */
    scrollToBottom() {
      const chatHistoryElement = this.$el.querySelector('header');
      if (chatHistoryElement) {
        chatHistoryElement.scrollTop = chatHistoryElement.scrollHeight;
      }
    }
  },
});
</script>

<style scoped>
/* 必要に応じてCSSを追加 */
</style>
