import { devtools } from '@vue/devtools'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './App.vue'
import './assets/main.css'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import router from './router';

if (process.env.NODE_ENV === 'development') {
  devtools.connect('http://localhost', 8098)
}
const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(ElementPlus)
app.use(router);
app.mount('#app')

