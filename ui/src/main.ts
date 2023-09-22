import 'assets/style/main.less'
import {Dimensions} from 'canvas/dimensions/game.ts'
import runner from 'canvas/runner.ts'
import dom from 'canvas/ports/operations/dom.ts'
import udom from 'util/udom.ts'

const game = Dimensions.build('dimensions', { parent: 'app' })
udom.onload(() => {
    dom.autoFullSize(game)
    return runner.run(game)
}).catch(err => {
    console.error(err)
})
