import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import "virtual:uno.css";
import { createRouter, createWebHistory } from "vue-router";

// Route
import MainPage from "./components/MainPage.vue";
import Login from "./components/Login.vue";
import Vote from "./components/Vote.vue";
import Niming from "./components/Niming.vue";
import Register from "./components/Register.vue";
import License from "./components/License.vue";
import Verify from "./components/Verify.vue";
import { createPinia } from "pinia";

const routes = [
  { path: "/", component: MainPage },
  { path: "/login", component: Login },
  { path: "/register", component: Register },
  { path: "/vote", component: Vote },
  { path: "/niming", component: Niming },
  { path: "/license", component: License },
  { path: "/verify", component: Verify },
];
const router = createRouter({
  history: createWebHistory(),
  routes,
});

const pinia = createPinia();

createApp(App).use(pinia).use(router).mount("#app");
