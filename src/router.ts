import { createRouter, createWebHistory, RouteRecordRaw, Router } from "vue-router";
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

const router: Router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
