<template>
  <form @submit.prevent="handleSubmit" class="input-form">
    <div class="form-group mb-3">
      <textarea v-model="input" class="form-control" placeholder="Ask ChatGPT..." rows="4"
        @keydown="checkCtrlEnter"></textarea>
    </div>
    <div class="d-flex justify-content-between align-items-center">
      <FileSelector @files-selected="handleFilesSelected" />
      <button type="submit" class="btn btn-primary">Send</button>
    </div>
  </form>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { convertFileToBase64 } from '../convertFileToBase64';
import { EncodedImage } from '../types';
import FileSelector from './FileSelector.vue';

export default defineComponent({
  components: { FileSelector },
  props: {
    onSubmit: Function as PropType<(input: string, base64Images?: EncodedImage[]) => void>,
  },
  data() {
    return {
      input: '',
      selectedFiles: [] as File[],
    };
  },
  methods: {
    async handleSubmit() {
      if (this.onSubmit && this.input.trim() !== '') {
        const base64Images: EncodedImage[] = [];

        for (const file of this.selectedFiles) {
          try {
            const base64Image = await convertFileToBase64(file);
            base64Images.push(base64Image);
          } catch (error) {
            console.error(`Error converting file ${file.name}:`, error);
          }
        }
        this.onSubmit(this.input, base64Images);
        this.input = '';
        this.selectedFiles = [];
      }
    },
    checkCtrlEnter(event: KeyboardEvent) {
      if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
        this.handleSubmit();
      }
    },
    handleFilesSelected(files: File[]) {
      this.selectedFiles = files;
    },
  },
  mounted() {

  },
});
</script>

<style scoped></style>
