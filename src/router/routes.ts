import { RouteRecordRaw } from "vue-router";

export const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "home",
    component: () => import("@/views/Accounts.vue"),
  },
  {
    path: "/accounts/add",
    name: "add-account",
    component: () => import("@/views/AccountForm.vue"),
  },
  {
    path: "/accounts/edit/:id",
    name: "edit-account",
    component: () => import("@/views/AccountForm.vue"),
  },
  {
    path: "/settings",
    name: "settings",
    component: () => import("@/views/Settings.vue"),
  },
];
