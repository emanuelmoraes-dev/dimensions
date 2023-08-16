import {IGame} from 'play/ports/i-game'

export default {
    loop(game: IGame): void {
        game.draw()
        requestAnimationFrame(() => this.loop(game))
    },
    async run(game: IGame): Promise<void> {
        await game.setup()
        this.loop(game)
    }
}
