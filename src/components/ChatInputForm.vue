<template>
  <form @submit.prevent="handleSubmit" class="input-form">
    <div class="form-group mb-3">
      <textarea
        v-model="input"
        class="form-control"
        placeholder="Ask ChatGPT..."
        rows="4"
        @keydown="checkCtrlEnter"
      ></textarea>
    </div>
    <div class="d-flex justify-content-between align-items-center">
      <input type="file" @change="handleFileChange" accept="image/*" class="form-control-file" />
      <button type="submit" class="btn btn-primary">Send</button>
    </div>
  </form>
</template>

<script lang="ts">
import { defineComponent, ref, PropType } from 'vue';
import { convertFileToBase64 } from '../convertFileToBase64';

export default defineComponent({
  props: {
    onSubmit: Function as PropType<(input: string, base64Image?: string) => void>,
  },
  setup(props) {
    const input = ref('');
    const selectedFile = ref<File | null>(null);

    const handleSubmit = async () => {
      if (props.onSubmit && input.value.trim() !== '') {
        let base64Image: string | undefined = undefined;

        if (selectedFile.value) {
          base64Image = await convertFileToBase64(selectedFile.value);
        }

        props.onSubmit(input.value, base64Image);
        input.value = '';
        selectedFile.value = null;
      }
    };

    const checkCtrlEnter = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
        handleSubmit();
      }
    };

    const handleFileChange = (event: Event) => {
      const target = event.target as HTMLInputElement;
      if (target.files && target.files[0]) {
        selectedFile.value = target.files[0];
      }
    };

    return {
      input,
      handleSubmit,
      checkCtrlEnter,
      handleFileChange,
    };
  },
});
</script>

<style scoped>
.input-form {
  background-color: #f8f9fa;
  padding: 15px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.form-control {
  resize: none;
}

.d-flex {
  margin-top: 10px;
}
</style>
