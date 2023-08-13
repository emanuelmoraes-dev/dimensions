export default {
    alias: {
        core: '../core',
        play: './src/play',
        types: './src/types',
        util: './util'
    },
    mount: {
        'style': '/style',
        'src': '/dist',
        '../core/pkg': { url: '/dist/core/pkg', static: true, resolve: false },
        'public': { url: '/', static: true, resolve: false }
    },
    exclude: ['**/package.json'],
    plugins: ['@snowpack/plugin-typescript', 'snowpack-plugin-less']
}
