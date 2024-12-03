<template>
    <div class="model-selector">
        <label for="model-select">Select Model:</label>
        <select v-model="localSelectedModel">
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
import { PropType } from 'vue';
import { ModelName } from '../types';

export default {
    props: {
        selectedModel: {
            type: [String, null] as PropType<ModelName | null>,
            default: null
        },
    },
    data() {
        return {
            availableModels: [] as string[],
        };
    },
    computed: {
        localSelectedModel: {
            get() {
                return this.selectedModel;
            },
            set(value: string) {
                this.$emit('update:selectedModel', value);
            }
        }
    },
    async created() {
        const defaultModel = await getDefaultModel();
        if (!this.selectedModel && defaultModel) {
            this.$emit('update:selectedModel', defaultModel);
        }
    },
    async mounted() {
        await this.fetchAvailableModels();
    },
    methods: {
        async fetchAvailableModels() {
            const apiKey = await getApiKey();
            if (!apiKey) return;

            const models = await getAvailableModels(apiKey);
            if (!models) return;

            this.availableModels = models;

            const defaultModel = await getDefaultModel();
            if (defaultModel && models.includes(defaultModel)) {
                this.$emit('update:selectedModel', defaultModel);
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
