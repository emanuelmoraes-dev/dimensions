import core from 'play/dimensions/core'
import dom from 'play/operations/dom'
import objUtil from 'util/obj-util'
import {X} from 'core/pkg/core'
import {IGame} from 'play/ports/i-game'
import {ICanvas} from 'play/ports/i-obj'
import {IGameConfig} from 'play/ports/i-config'
import {DeepPartial} from 'types'

const buildGameConfig = (config: DeepPartial<IGameConfig>): IGameConfig => ({
    grid: objUtil.setDefault(config.grid ?? {}, {
        minImageWidth: 50,
        maxImageWidth: 100,

        minImageHeigth: 50,
        maxImageHeigth: 100,

        percentImageWidth: 0.01,
        percentIMageHeigth: 0.01
    })
})

export class Dimensions implements IGame {
    width!: number
    height!: number

    canvas: ICanvas = dom.createCanvas(this.id)
    config: IGameConfig

    private x!: X

    constructor(
        public id: string,
        private nickname: string,
        private description: string,
        config: DeepPartial<IGameConfig>
    ) {
        this.config = buildGameConfig(config)
    }

    async setup(): Promise<void> {
        this.x = await core.init(this.nickname, this.description)
        dom.append(this.canvas)
    }

    draw(): void {
        const canvas = this.canvas
        const context = canvas.context
        const canvasElement = canvas.element
        context.clearRect(0, 0, canvasElement.width, canvasElement.height)
        context.fillStyle = 'black'
        context.fillRect(0, 0, canvasElement.width, canvasElement.height)
    }

    static build(id: string, config: DeepPartial<IGameConfig> = {}) {
        return new Dimensions(id, 'Nickname', 'Description', config)
    }
}
