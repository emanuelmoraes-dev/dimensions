import { IGame } from 'game/ports/i-game'

export default {
    setup(game: IGame) {
        game.config().catch(err => console.error(err))
    },
    draw(game: IGame) {
        game.draw()
        requestAnimationFrame(() => game.draw())
    }
}
