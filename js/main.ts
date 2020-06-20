import axios from "axios"
const http = axios.create({
    baseURL: "http://" + location.host
})

http.get("/sample2.html").then((res) => {

    import("../pkg/index.js").then(wasm => {
        const json = wasm.parse(res.data)
        console.log(JSON.parse(json))
    }).catch(console.error);

})

import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')