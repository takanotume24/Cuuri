import { createRouter, createWebHistory } from "npm:vue-router";
import HomeView from "./view/HomeView.vue";
import SettingsView from "./view/SettingsView.vue";

const routes = [
  {
    path: "/",
    component: HomeView,
  },
  {
    path: "/settings",
    component: SettingsView,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
