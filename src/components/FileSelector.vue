<template>
    <div>
        <!-- Hidden default file input -->
        <input type="file" @change="handleFileChange" accept="image/*" multiple class="form-control-file"
            ref="fileInput" style="display: none" />
        <!-- Custom button for file selection -->
        <button type="button" class="btn btn-secondary" @click="triggerFileInput">
            Select Files
        </button>
        <div v-if="fileNames.length" class="mt-2">
            <strong>Selected files:</strong>
            <ul>
                <li v-for="fileName in fileNames" :key="fileName">{{ fileName }}</li>
            </ul>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
    emits: ['files-selected'],
    setup(_, { emit }) {
        const selectedFiles = ref<File[]>([]);
        const fileNames = ref<string[]>([]);
        const fileInput = ref<HTMLInputElement | null>(null);

        const handleFileChange = (event: Event) => {
            const target = event.target as HTMLInputElement;
            if (target.files) {
                selectedFiles.value = Array.from(target.files);
                fileNames.value = selectedFiles.value.map(file => file.name);
                emit('files-selected', selectedFiles.value);
            }
        };

        const triggerFileInput = () => {
            fileInput.value?.click();
        };

        return {
            fileNames,
            handleFileChange,
            triggerFileInput,
            fileInput,
        };
    },
});
</script>

<style scoped></style>