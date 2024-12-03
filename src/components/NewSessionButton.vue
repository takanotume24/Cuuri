<template>
    <button class="btn btn-outline-secondary mb-3" @click="$emit('new-session')">New Session</button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'NewSessionButton',
});
</script>

<style scoped></style>
