import { resolve } from 'node:path'
import { defineConfig } from 'vite'

export default defineConfig({
    // CHanging the root directory to src
    root: resolve(import.meta.dirname, "src"),

    build: {
        // The dist dir should be outside the src dir, so put it one dir lower
        outDir: "../dist",
        emptyOutDir: true,

        // I don't like how unreadable minified code is
        minify: false,

        rollupOptions: {
            input: {
                index: resolve(import.meta.dirname, "src/index.html"),
                login: resolve(import.meta.dirname, "src/login.html"),
                dashboard: resolve(import.meta.dirname, "src/dashboard.html"),
                artist: resolve(import.meta.dirname, "src/artist.html"),
                album: resolve(import.meta.dirname, "src/album.html"),
                recent: resolve(import.meta.dirname, "src/recent.html")
            }
        }
    },
})
