import {ICanvas, IElement} from 'canvas/ports/i-obj.ts'
import {IGame} from 'canvas/ports/i-game.ts'
import config from 'canvas/ports/operations/config.ts'

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
        config.setSize(game, width, height)
    },
    autoFullSize(game: IGame): void {
        this.setFullSize(game)
        window.addEventListener('resize', () => {
            this.setFullSize(game)
            game.draw()
        })
    }
}
