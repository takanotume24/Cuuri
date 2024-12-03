<template>
    <ul class="list-group list-group-flush flex-grow-1 overflow-auto mb-3">
        <li v-for="sessionId in reversedSessionIdList" :key="sessionId"
            :class="['list-group-item', 'cursor-pointer', { active: currentSessionId === sessionId }]"
            @click="selectSession(sessionId)">
            Chat Session: {{ sessionId }}
        </li>
    </ul>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { SessionId } from '../types';

export default defineComponent({
    props: {
        sessionIdList: {
            type: Array as PropType<SessionId[]>,
            required: true,
        },
        currentSessionId: {
            type: [String, null] as PropType<SessionId | null>,
            default: null,
        },
    },
    computed: {
        reversedSessionIdList(): SessionId[] {
            return [...this.sessionIdList].reverse();
        },
    },
    methods: {
        selectSession(sessionId: SessionId) {
            this.$emit('select-session', sessionId);
        },
    },
});
</script>

<style scoped></style>
