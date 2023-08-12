import core from './core'
import dom from 'game/operations/dom'
import { Dimensions } from 'core/pkg/core'
import { IGame } from 'game/ports/i-game'
import { ICanvas } from 'game/ports/i-obj'

export class Game implements IGame {
    canvas: ICanvas = dom.createCanvas(this.id, this.width, this.heigth);

    private dimensions?: Dimensions;

    constructor(
        public id: string,
        public width: number,
        public heigth: number,
        private nickname: string,
        private description: string
    ) {}

    async config(): Promise<void> {
        this.dimensions = await core.init(this.nickname, this.description)
        dom.append(this.canvas)
    }

    draw(): void {
        const canvas = this.canvas
        canvas.context.clearRect(0, 0, canvas.width, canvas.height)
        canvas.context.fillStyle = 'black'
        canvas.context.fillRect(0, 0, canvas.width, canvas.height)
    }

    static build(id: string, width: number, heigth: number) {
        return new Game(id, width, heigth, 'Nickname', 'Description')
    }
}
