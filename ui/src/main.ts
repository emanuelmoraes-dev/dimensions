import {Dimensions} from 'canvas/dimensions/game'
import runner from 'canvas/runner'
import dom from 'canvas/ports/operations/dom'
import udom from 'util/udom'

const game = Dimensions.build('dimensions')
udom.onload(async () => {
    dom.autoFullSize(game)
    return runner.run(game)
}).catch(err => {
    console.error(err)
})
