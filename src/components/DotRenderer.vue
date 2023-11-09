<template>
  <div class="container">
    <img :src="img" v-if="code.length !== 0 && !hasError" />
    <div v-if="hasError">
      {{ errorMessage }}
    </div>
    <button @click="saveSVG(svg)" class="float-btn">Save</button>
  </div>
</template>

<script setup lang="ts">
import { Graphviz } from "@hpcc-js/wasm/graphviz";
import { onMounted, ref, watch, computed, type Ref } from "vue";
import { saveSVG } from "./bridge";

const props = defineProps<{
  code: string;
  hasError: boolean;
  errorMessage: string;
}>();

const svg: Ref<string> = ref("");
const graphviz = ref();

const update = async () => {
  try {
    if (graphviz.value) {
      svg.value = await graphviz.value.dot(props.code);
    }
  } catch (e) {
    console.error(e);
  }
};

onMounted(async () => {
  graphviz.value = await Graphviz.load();
  await update();
});

watch(props, async () => {
  await update();
});

const img = computed(() => {
  return `data:image/svg+xml;base64, ${btoa(svg.value)}`;
});
</script>

<style>
.container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: 90vh;
  width: 100%;
}

img {
  height: 100%;
  object-fit: scale-down;
}

.float-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  width: 56px;
  height: 56px;
  padding: 0px;
  border: none;
  cursor: pointer;
  user-select: none;
  overflow: hidden;
  position: fixed;
  right: 16px;
  bottom: 16px;
  border-radius: 50%;
}
</style>
