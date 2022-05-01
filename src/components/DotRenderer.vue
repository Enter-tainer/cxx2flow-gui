<template>
  <div class="fa">
    <img :src="img" id="img" />
  </div>
</template>

<script setup lang="ts">
import { graphviz, wasmFolder } from '@hpcc-js/wasm'
import { defineProps, onMounted, ref, watch, computed } from 'vue'

const props = defineProps<{
  code: string
}>()
const svg = ref<string>('')
const update = async () => {
  try {
    const res = await graphviz.dot(props.code)
    svg.value = res
  } catch (e) {
    console.error(e)
  }
}
onMounted(async () => {
  wasmFolder('/')
  await update()
})
watch(props, async () => {
  await update()
})
const img = computed(() => {
  return `data:image/svg+xml;base64, ${btoa(svg.value)}`
})
</script>

<style>
.fa {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

#img {
  margin: 0 auto;
}
</style>
