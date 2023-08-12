import {ICanvas, IElement} from 'play/ports/i-obj'
import {IGame} from 'play/ports/i-game'

export default {
    append(...elements: IElement[]): void {
        elements.forEach(e => e.parent.appendChild(e.element))
    },
    createCanvas(id: string): ICanvas {
        const element = document.createElement('canvas')
        element.id = id
        return {
            id,
            parent: document.body,
            element: element,
            context: element.getContext('2d') as CanvasRenderingContext2D
        }
    },
    setFullSize(game: IGame): void {
        const width = document.documentElement.clientWidth
        const height = document.documentElement.clientHeight

        game.width = width
        game.height = height

        const canvasElement = game.canvas.element
        canvasElement.width = width
        canvasElement.height = height
    },
    autoFullSize(game: IGame): void {
        this.setFullSize(game)
        window.addEventListener('resize', () => {
            this.setFullSize(game)
            game.draw()
        })
    },
    onload(callback: () => void): void {
        document.addEventListener('DOMContentLoaded', callback)
    }
}
