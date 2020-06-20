import("../pkg/index.js").then(wasm => {
    const json = wasm.parse(`
    

    
    `)

    console.log(JSON.parse(json))
}).catch(console.error);

import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')