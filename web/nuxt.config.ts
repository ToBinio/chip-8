import wasm from "vite-plugin-wasm";

export default defineNuxtConfig({
    vite: {
        plugins: [wasm()],
    },
    compatibilityDate: '2024-11-01',
    devtools: {enabled: true}
})
