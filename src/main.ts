import { createApp } from "npm:vue";
import App from "./App.vue";
import { devtools } from "npm:@vue/devtools";
import router from "./router.ts";
import "npm:bootstrap/dist/css/bootstrap.css";

if (Deno.env.get("NODE_ENV") === "development") {
  devtools.connect();
}

createApp(App).use(router).mount("#app");
