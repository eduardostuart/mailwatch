import { RouteRecordRaw } from "vue-router";
import AccountsView from "@/views/Accounts.vue";
import AccountFormView from "@/views/AccountForm.vue";
import SettingsView from "@/views/Settings.vue";

export const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "home",
    component: AccountsView,
  },
  {
    path: "/accounts/add",
    name: "add-account",
    component: AccountFormView,
  },
  {
    path: "/accounts/edit/:id",
    name: "edit-account",
    component: AccountFormView,
  },
  {
    path: "/settings",
    name: "settings",
    component: SettingsView,
  },
];
