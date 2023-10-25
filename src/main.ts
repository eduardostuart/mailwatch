import "@/styles.css";
import { createApp } from "vue";
import { registerRouter } from "@/router";
import App from "@/App.vue";

createApp(App).use(registerRouter()).mount("#app");
