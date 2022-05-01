import { invoke } from '@tauri-apps/api/tauri'

const generateGraphviz = async (code: string): Promise<string> => {
  const res = await invoke('generate_graphviz', {
    code
  })
  console.log(res)
  return String(res)
}

export { generateGraphviz }
