<template>
  <div class="split">
    <CodeEditor id="split-0" v-model:code="code"></CodeEditor>
    <DotRenderer id="split-1" :code="dotCode"></DotRenderer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import Split from 'split.js'
import CodeEditor from './CodeEditor.vue'
import DotRenderer from './DotRenderer.vue'
import { throttledWatch } from '@vueuse/shared'
import { generateGraphviz } from './bridge'
onMounted(() => {
  Split(['#split-0', '#split-1'])
})
const code = ref<string>(`
#include <iostream>
using namespace std;
int main() {
  int a, b;
  cin >> a >> b;
  cout << a + b << endl;
}
`.trimStart()
)
const dotCode = ref<string>('')
watch(code, () => {
  // console.log(code.value)
})

throttledWatch(code, async () => {
  dotCode.value = await generateGraphviz(code.value)
}, { throttle: 500 })
</script>

<style>
.split {
  display: flex;
  flex-direction: row;
}

.gutter {
  background-color: #eee;
  background-repeat: no-repeat;
  background-position: 50%;
}

.gutter.gutter-horizontal {
  background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAeCAYAAADkftS9AAAAIklEQVQoU2M4c+bMfxAGAgYYmwGrIIiDjrELjpo5aiZeMwF+yNnOs5KSvgAAAABJRU5ErkJggg==');
  cursor: col-resize;
}
</style>
