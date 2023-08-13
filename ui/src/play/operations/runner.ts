import {IGame} from 'play/ports/i-game'

export default {
    async run(game: IGame): Promise<void> {
        await game.setup()
        game.draw()
        requestAnimationFrame(() => game.draw())
    }
}
