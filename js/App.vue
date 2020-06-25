<template>
  <input type="number" v-model="num" />
  <select v-model="baseURL" @click="clearThread">
    <option disabled value>板</option>
    <option value="http://192.168.1.2:8081/mock-img">img</option>
    <option value="http://192.168.1.2:8081/mock-may">may</option>
  </select>
  <input type="button" value="表示" @click="getThread" />
  <div v-for="(c,i) in contributions" :key="c.quote">
    <span>{{i}} {{c.title}} {{c.name}} {{c.date}} {{c.id}}</span>
    <div v-html="c.quote" />
    <img v-for="j in c.images" :src="j" />
  </div>
</template>

<script lang="ts">
import { ref, defineComponent } from "vue";
import axios from "axios";

interface Contribution {
  images: String[];
  quote: String;
  name: String;
  title: String;
  id: String;
  sod: String;
  date: String;
}

interface Thread {
  contributions: Contribution[];
}

export default defineComponent({
  setup() {
    const baseURL = ref<String>("http://192.168.1.2:8081/mock-img");

    const num = ref<Number>();
    const contributions = ref<Contribution[]>();

    const getThread = () => {
      axios
        .get(["b", "res", num.value + ".htm"].join("/"), {
          baseURL: baseURL.value.toString()
        })
        .then(res => {
          import("../pkg/index.js")
            .then(wasm => {
              const json = wasm.parse(res.data, baseURL.value.toString());
              const thread: Thread = JSON.parse(json);
              contributions.value = thread.contributions;
            })
            .catch(console.error);
        });
    };

    const clearThread = () => {
      contributions.value = [
        {
          images: [""],
          quote: "",
          name: "",
          title: "",
          id: "",
          sod: "",
          date: ""
        }
      ];
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
