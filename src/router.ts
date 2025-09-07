import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeView from "./view/HomeView.vue";
import SettingsView from "./view/SettingsView.vue";

const routes: RouteRecordRaw[] = [
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
