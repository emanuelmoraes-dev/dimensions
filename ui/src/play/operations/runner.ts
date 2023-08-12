import {IGame} from 'play/ports/i-game'
import dom from 'play/operations/dom'

export default {
    async run(game: IGame): Promise<void> {
        await game.config()
        game.draw()
        requestAnimationFrame(() => game.draw())
    },
    load(game: IGame) {
        dom.onload(() => {
            this.run(game).catch(err => console.error(err))
        })
    }
}
