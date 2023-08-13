import {Dimensions} from 'play/dimensions/game'
import runner from 'play/operations/runner'
import dom from 'play/operations/dom'
import domUtil from 'util/dom-util'

const game = Dimensions.build('dimensions')
domUtil.onload(async () => {
    dom.autoFullSize(game)
    return runner.run(game)
}).catch(err => {
    console.error(err)
})
