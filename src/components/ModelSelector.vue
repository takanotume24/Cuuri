<template>
    <div class="model-selector">
        <label for="model-select">Select Model:</label>
        <select v-model="internalSelectedModel" @change="updateValue">
            <option v-for="model in availableModels" :key="model" :value="model">
                {{ model }}
            </option>
        </select>
    </div>
</template>

<script lang="ts">
import { getAvailableModels } from './../getAvailableModels';
import { getDefaultModel } from './../getDefaultModel';
import { getApiKey } from './../getApiKey';

export default {
    props: {
        selectedModel: {
            type: String,
            default: '',
        },
        isApiKeySet: {
            type: Boolean,
            required: true,
        },
    },
    data() {
        return {
            availableModels: [] as string[],
            internalSelectedModel: this.selectedModel
        };
    },
    async mounted() {
        await this.fetchAvailableModels();
    },
    watch: {
        isApiKeySet(v: boolean) {
            if (v) {
                this.fetchAvailableModels();
            }
        },
        selectedModel(newVal: string) {
            this.internalSelectedModel = newVal;
        },
    },
    methods: {
        updateValue(event: Event) {
            const target = event.target as HTMLSelectElement;
            this.$emit('update:selectedModel', target.value);
        },
        async fetchAvailableModels() {
            const apiKey = await getApiKey();
            if (!apiKey) {
                return;
            }

            const models = await getAvailableModels(apiKey);
            if (!models) {
                return;
            }

            this.availableModels = models;
            if (models.length > 0) {
                const defaultModel = await getDefaultModel();
                if (!defaultModel) {
                    this.$emit('update:selectedModel', '');
                    return;
                }
                this.internalSelectedModel = models.includes(defaultModel) ? defaultModel : '';
                this.$emit('update:selectedModel', this.internalSelectedModel);
            }
        },
    },
};
</script>

<style>
.model-selector {
    margin-top: auto;
}
</style>