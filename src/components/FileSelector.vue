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
import { defineComponent, ref, watch } from 'vue';

export default defineComponent({
    props: {
        modelValue: {
            type: Array as () => File[],
            default: () => []
        }
    },
    emits: ['update:modelValue'],
    setup(props, { emit }) {
        const fileNames = ref<string[]>([]);
        const fileInput = ref<HTMLInputElement | null>(null);

        // Watch for changes in modelValue and update fileNames
        watch(() => props.modelValue, (newFiles) => {
            fileNames.value = newFiles.map(file => file.name);
        }, { immediate: true });

        const handleFileChange = (event: Event) => {
            const target = event.target as HTMLInputElement;
            if (target.files) {
                const files = Array.from(target.files);
                emit('update:modelValue', files);
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