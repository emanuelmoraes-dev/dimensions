export default {
    alias: {
        core: '../core',
        canvas: './src/canvas',
        types: './src/types',
        util: './src/util'
    },
    mount: {
        'style': '/style',
        'src': '/dist',
        '../core/pkg': {url: '/dist/core/pkg', static: true, resolve: false},
        'public': {url: '/', static: true, resolve: false}
    },
    exclude: ['**/package.json'],
    plugins: ['@snowpack/plugin-typescript', 'snowpack-plugin-less']
}
