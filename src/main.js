import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { ref } from "vue";

const app = createApp(App);

// 设置
export const config = ref(null);

app.use(ElementPlus);
app.mount("#app");