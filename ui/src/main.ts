import {Dimensions} from 'play/dimensions/game'
import runner from 'play/operations/runner'
import dom from 'play/operations/dom'
import udom from './util/udom'

const game = Dimensions.build('dimensions')
udom.onload(async () => {
    dom.autoFullSize(game)
    return runner.run(game)
}).catch(err => {
    console.error(err)
})
