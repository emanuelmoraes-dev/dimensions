export default {
    mount: {
        'src': '/dist',
        '../core/pkg': { url: '/dist/core/pkg', static: true, resolve: false },
        'public': { url: '/', static: true, resolve: false }
    },
    exclude: ['**/package.json']
}
