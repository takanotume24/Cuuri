<template>
    <div id="chat-history">
        <div v-for="(entry, index) in chatHistory" :key="index" class="chat-entry">
            <div class="user-message">
                <strong>You:</strong>
                <pre>{{ entry.question }}</pre>
            </div>
            <div class="gpt-response" v-html="entry.answer"></div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HtmlChatEntry } from '../types';

export default defineComponent({
    props: {
        chatHistory: Array as PropType<HtmlChatEntry[]>,
    },
});
</script>

<style scoped>
#chat-history {
    flex-grow: 1;
    overflow-y: auto;
    border-bottom: 1px solid #ccc;
    margin-bottom: 10px;
    padding-right: 10px;
}

.chat-entry {
    margin-bottom: 10px;
}

.user-message {
    margin-bottom: 5px;
}
</style>