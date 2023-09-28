import {IGame} from 'canvas/ports/i-game.ts'

export default {
    loop(game: IGame): void {
        game.loop()
        requestAnimationFrame(() => this.loop(game))
    },
    async run(game: IGame): Promise<void> {
        await game.setup()
        this.loop(game)
    }
}
