import { createApp } from "vue";
import App from "./App.vue";
import ApiKeyDialog from "./components/ApiKeyDialog.vue";
import ModelSelector from "./components/ModelSelector.vue";

createApp(App)
  .component("ApiKeyDialog", ApiKeyDialog)
  .component("ModelSelector", ModelSelector)
  .mount("#app");
