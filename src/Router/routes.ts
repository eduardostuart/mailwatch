import { RouteRecordRaw } from "vue-router";
import AccountsView from "@/Views/Accounts.vue";
import AccountFormView from "@/Views/AccountForm.vue";
import SettingsView from "@/Views/Settings.vue";

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
