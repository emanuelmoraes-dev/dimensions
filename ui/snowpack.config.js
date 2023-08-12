export default {
    alias: {
        core: '../core',
        game: './src/game'
    },
    mount: {
        'src': '/dist',
        '../core/pkg': { url: '/dist/core/pkg', static: true, resolve: false },
        'public': { url: '/', static: true, resolve: false }
    },
    exclude: ['**/package.json'],
    plugins: ['@snowpack/plugin-typescript']
}
