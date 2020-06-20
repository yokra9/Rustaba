<template>
  <pre>{{ tmp }}</pre>
</template>

<script lang="ts">
import { ref, defineComponent } from "vue";
import axios from "axios";

export default defineComponent({
  setup() {
    const http = axios.create({
      baseURL: "http://" + location.host
    });

    const tmp = ref<String>();
    http.get("/sample3.html").then(res => {
      import("../pkg/index.js")
        .then(wasm => {
          const json = wasm.parse(res.data);
          tmp.value = JSON.parse(json);
        })
        .catch(console.error);
    });

    return {
      tmp
    };
  }
});
</script>

<style scoped>
</style>
