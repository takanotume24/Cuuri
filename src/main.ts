import { createApp } from "vue";
import App from "./App.vue";
import ApiKeyDialog from "./components/ApiKeyDialog.vue";
import ModelSelector from "./components/ModelSelector.vue";
import { devtools } from "@vue/devtools";
import "bootstrap/dist/css/bootstrap.css";

if (process.env.NODE_ENV === "development") {
  devtools.connect();
}

createApp(App)
  .component("ApiKeyDialog", ApiKeyDialog)
  .component("ModelSelector", ModelSelector)
  .mount("#app");
