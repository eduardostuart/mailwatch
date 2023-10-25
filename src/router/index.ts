import { createRouter, createWebHistory, Router } from "vue-router";
import { baseUrl } from "@/config";
import { routes } from "./routes";

export function registerRouter(): Router {
  return createRouter({
    history: createWebHistory(baseUrl),
    routes,
  });
}
