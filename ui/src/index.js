import init, { Dimensions, run } from '../../core/pkg/core.js'

await init()

const game = new Dimensions('Nickname', 'Description')
run(game)
