<template>
  <Codemirror v-model:value="codeData" :options="cmOptions" border :height="height - 24" class="editor" />
</template>

<script setup lang="ts">
import "codemirror/mode/javascript/javascript.js";
import Codemirror from "codemirror-editor-vue3";
import type { EditorConfiguration } from "codemirror";
import { useWindowSize, useVModel } from "@vueuse/core";

const props = defineProps<{
  code: string;
}>();
const emit = defineEmits(["update:code"]);

const codeData = useVModel(props, "code", emit);
const { height } = useWindowSize();
const cmOptions: EditorConfiguration = {
  mode: "text/x-c++src", // Language mode
  lineNumbers: true, // Show line number
  smartIndent: true, // Smart indent
  indentUnit: 2, // The smart indent unit is 2 spaces in length
  foldGutter: true, // Code folding
  styleActiveLine: true // Display the style of the selected row
};
</script>
<style scoped>
.editor:deep(.CodeMirror) {
  font-size: 14px;
  font-family:
    Fira Code,
    JetBrains Mono,
    Consolas,
    monospace !important;
}
</style>
