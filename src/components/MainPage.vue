<template>
  <div class="split">
    <CodeEditor id="split-0" v-model:code="code"></CodeEditor>
    <DotRenderer id="split-1" :code="dotCode" :error-message="errorMessage" :has-error="hasError"></DotRenderer>
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
const errorMessage = ref<string>('')
const hasError = ref<boolean>(false)
const update = async () => {
  try {
    dotCode.value = await generateGraphviz(code.value)
    hasError.value = false
  } catch (e) {
    hasError.value = true
    errorMessage.value = String(e)
  }
}
onMounted(() => {
  update()
})
throttledWatch(code, async () => {
  update()
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
