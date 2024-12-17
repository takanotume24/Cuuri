import { createApp } from "vue";
import App from "./App.vue";
import { devtools } from "@vue/devtools";
import router from "./router";
import "bootstrap/dist/css/bootstrap.css";

if (process.env.NODE_ENV === "development") {
  devtools.connect();
}

createApp(App).use(router).mount("#app");
