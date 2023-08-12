import core from 'play/dimensions/core'
import dom from 'play/operations/dom'
import {X} from 'core/pkg/core'
import {IGame} from 'play/ports/i-game'
import {ICanvas} from 'play/ports/i-obj'

export class Dimensions implements IGame {
    width!: number
    height!: number
    canvas: ICanvas = dom.createCanvas(this.id)

    private x!: X

    constructor(
        public id: string,
        private nickname: string,
        private description: string
    ) {}

    async config(): Promise<void> {
        this.x = await core.init(this.nickname, this.description)
        dom.append(this.canvas)
        dom.autoFullSize(this)
    }

    draw(): void {
        const canvas = this.canvas
        const context = canvas.context
        const canvasElement = canvas.element
        context.clearRect(0, 0, canvasElement.width, canvasElement.height)
        context.fillStyle = 'black'
        context.fillRect(0, 0, canvasElement.width, canvasElement.height)
    }

    static build(id: string) {
        return new Dimensions(id, 'Nickname', 'Description')
    }
}
