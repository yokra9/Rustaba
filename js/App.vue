<template>
  <input type="number" v-model="num" />
  <select v-model="baseURL" @click="clearThread">
    <option disabled value>板</option>
    <option value="http://192.168.1.2:8081/mock-img">img</option>
    <option value="http://192.168.1.2:8081/mock-may">may</option>
  </select>
  <input type="button" value="表示" @click="getThread" />
  <div v-for="(c,i) in contributions" :key="c.quote">
    <span>{{i}} {{c.title}} {{c.name}} {{c.date}}</span>
    <div v-html="c.quote" />
    <img v-for="j in c.images" :src="j" />
  </div>
</template>

<script lang="ts">
import { ref, computed, defineComponent } from "vue";
import axios from "axios";

export default defineComponent({
  setup() {
    const baseURL = ref("http://192.168.1.2:8081/mock-img");

    const num = ref<Number>();
    const contributions = ref();

    const getThread = () => {
      axios
        .get(["b", "res", num.value + ".htm"].join("/"), {
          baseURL: baseURL.value
        })
        .then(res => {
          import("../pkg/index.js")
            .then(wasm => {
              const json = wasm.parse(res.data, baseURL.value);
              contributions.value = Object.assign(
                {},
                JSON.parse(json).contributions,
                contributions.value
              );
            })
            .catch(console.error);
        });
    };

    const clearThread = () => {
      contributions.value = {};
    };

    return {
      baseURL,
      num,
      contributions,
      getThread,
      clearThread
    };
  }
});
</script>

<style scoped>
</style>
