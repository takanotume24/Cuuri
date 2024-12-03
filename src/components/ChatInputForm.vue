<template>
    <form @submit.prevent="handleSubmit" class="input-form">
        <textarea v-model="input" placeholder="Ask ChatGPT..." rows="4" cols="50" @keydown="checkCtrlEnter"></textarea>
        <button type="submit">Send</button>
    </form>
</template>

<script lang="ts">
import { defineComponent, ref, PropType } from 'vue';

export default defineComponent({
    props: {
        onSubmit: Function as PropType<(input: string) => void>,
    },
    setup(props) {
        const input = ref('');

        const handleSubmit = () => {
            if (props.onSubmit && input.value.trim() !== '') {
                props.onSubmit(input.value);
                input.value = '';
            }
        };

        const checkCtrlEnter = (event: KeyboardEvent) => {
            if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
                handleSubmit();
            }
        };

        return {
            input,
            handleSubmit,
            checkCtrlEnter,
        };
    },
});
</script>

<style scoped>
.input-form {
    display: flex;
    align-items: center;
}

textarea {
    flex-grow: 1;
    margin-right: 10px;
    padding: 10px;
    font-size: 14px;
    border: 1px solid #ccc;
    border-radius: 4px;
    resize: none;
}

button[type="submit"] {
    padding: 10px 20px;
    border: none;
    background-color: #007bff;
    color: white;
    cursor: pointer;
    border-radius: 4px;
}

button[type="submit"]:hover {
    background-color: #0056b3;
}
</style>