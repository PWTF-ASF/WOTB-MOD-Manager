import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './App.vue'
import './assets/main.css'
import './styles/tokens.css'
import './styles/themes.css'
import './styles/base.css'
import './styles/motion.css'
import router from './router';

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router);
app.mount('#app')

