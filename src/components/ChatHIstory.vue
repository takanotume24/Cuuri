<template>
    <div id="chat-history" class="overflow-auto border-bottom mb-3 pr-3">
        <div v-for="(entry, index) in chatHistory" :key="index" class="chat-entry mb-3">
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
import { HtmlChatEntry } from '../types';

export default defineComponent({
    props: {
        chatHistory: Array as PropType<HtmlChatEntry[]>,
    },
});
</script>

<style scoped>
#chat-history {
    max-height: 500px; /* Example height, adjust as needed */
}
</style>
