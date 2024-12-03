<template>
  <form @submit.prevent="handleSubmit" class="input-form">
    <div class="form-group mb-3">
      <textarea v-model="input" class="form-control" placeholder="Ask ChatGPT..." rows="4"
        @keydown="checkCtrlEnter"></textarea>
    </div>
    <div class="d-flex justify-content-between align-items-center">
      <!-- Hidden default file input -->
      <input type="file" @change="handleFileChange" accept="image/*" multiple class="form-control-file" ref="fileInput"
        style="display: none" />
      <!-- Custom button for file selection -->
      <button type="button" class="btn btn-secondary" @click="triggerFileInput">
        Select Files
      </button>
      <button type="submit" class="btn btn-primary">Send</button>
    </div>
    <div v-if="fileNames.length" class="mt-2">
      <strong>Selected files:</strong>
      <ul>
        <li v-for="fileName in fileNames" :key="fileName">{{ fileName }}</li>
      </ul>
    </div>
  </form>
</template>


<script lang="ts">
import { defineComponent, ref, PropType } from 'vue';
import { convertFileToBase64 } from '../convertFileToBase64';
import { EncodedImage } from '../types';

export default defineComponent({
  props: {
    onSubmit: Function as PropType<(input: string, base64Images?: EncodedImage[]) => void>,
  },
  setup(props) {
    const input = ref('');
    const selectedFiles = ref<File[]>([]);
    const fileNames = ref<string[]>([]);
    const fileInput = ref<HTMLInputElement | null>(null);

    const handleSubmit = async () => {
      if (props.onSubmit && input.value.trim() !== '') {
        const base64Images: EncodedImage[] = [];

        for (const file of selectedFiles.value) {
          try {
            const base64Image = await convertFileToBase64(file);
            base64Images.push(base64Image);
          } catch (error) {
            console.error(`Error converting file ${file.name}:`, error);
          }
        }
        props.onSubmit(input.value, base64Images);
        input.value = '';
        selectedFiles.value = [];
        fileNames.value = [];
      }
    };

    const checkCtrlEnter = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
        handleSubmit();
      }
    };

    const handleFileChange = (event: Event) => {
      const target = event.target as HTMLInputElement;
      if (target.files) {
        selectedFiles.value = Array.from(target.files);
        fileNames.value = selectedFiles.value.map(file => file.name);
      }
    };

    const triggerFileInput = () => {
      fileInput.value?.click();
    };

    return {
      input,
      fileNames,
      handleSubmit,
      checkCtrlEnter,
      handleFileChange,
      triggerFileInput,
      fileInput,
    };
  },
});
</script>


<style scoped></style>
