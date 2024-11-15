import { defineConfig } from "vitest/config";
import wasm from 'vite-plugin-wasm';
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait(),
    ],
    optimizeDeps: {
        exclude: [
            "fitl-wasm"
        ]
    },
    assetsInclude: ['**/*.wasm'],
    build: {
        rollupOptions: {
            output: {
                manualChunks: undefined, // Avoid issues with code-splitting
            },
        },
    },
    test: {
        globals: true,
        environment: "node",
    },
});
