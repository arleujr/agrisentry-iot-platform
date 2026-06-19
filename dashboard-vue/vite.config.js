import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// Configuration matrix mapping out how Vite processes Single-File Components
export default defineConfig({
  plugins: [vue()],
})