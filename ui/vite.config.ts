import {defineConfig} from 'vite'

export default defineConfig({
    resolve: {
        alias: {
            'assets': '/assets',
            'canvas': '/src/canvas',
            'types': '/src/types',
            'util': '/src/util'
        }
    }
})
