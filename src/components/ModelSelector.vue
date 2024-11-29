<template>
    <div class="model-selector">
        <label for="model-select">Select Model:</label>
        <select id="model-select" v-model="selectedModel">
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
        isApiKeySet: {
            type: Boolean,
            required: true
        }
    },
    data() {
        return {
            selectedModel: '',
            availableModels: [] as string[],
        };
    },
    async mounted() {
        await this.fetchAvailableModels();
    },
    watch: {
        isApiKeySet(v: boolean) {
            if (v) {
                this.fetchAvailableModels()
            }
        }
    },
    methods: {
        async fetchAvailableModels() {
            const apiKey = await getApiKey();
            if (!apiKey) {
                return;
            };

            const models = await getAvailableModels(apiKey);
            if (!models) {
                return;
            }

            this.availableModels = models;
            if (models.length > 0) {
                const defaultModel = await getDefaultModel();
                if (!defaultModel) {
                    this.selectedModel = '';
                    return;
                }
                this.selectedModel = models.includes(defaultModel) ? defaultModel : '';
            }
        },
    }

}
</script>

<style>
.model-selector {
    margin-top: auto;
}
</style>