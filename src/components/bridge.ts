import { invoke } from '@tauri-apps/api/tauri'
import { save } from '@tauri-apps/api/dialog'
import { writeFile } from '@tauri-apps/api/fs'

const generateGraphviz = async (code: string): Promise<string> => {
  const res = await invoke('generate_graphviz', {
    code
  })
  console.log(res)
  return String(res)
}

const saveSVG = async (svg: string): Promise<void> => {
  try {
    const path = await save({ filters: [{ extensions: ['svg'], name: 'SVG Files' }], title: 'Save your SVG file' })
    await writeFile({
      path,
      contents: svg
    })
  } catch (e) {
    console.log(e)
  }
}

export { generateGraphviz, saveSVG }
