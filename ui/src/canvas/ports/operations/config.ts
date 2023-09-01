import {IGame} from 'canvas/ports/i-game.ts'

export default {
    setSize(game: IGame, width: number, height: number): void {
        game.width = width
        game.height = height

        const canvasElement = game.canvas.element
        canvasElement.width = width
        canvasElement.height = height
    }
}
