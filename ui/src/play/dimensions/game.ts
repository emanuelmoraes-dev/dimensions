import core from 'play/dimensions/core'
import dom from 'play/operations/dom'
import uobj from 'util/uobj'
import {Grid} from 'play/dimensions/grid'
import {X} from 'core/pkg/core'
import {IGame} from 'play/ports/i-game'
import {ICanvas} from 'play/ports/i-obj'
import {IGameConfig} from 'play/ports/i-config'
import {DeepPartial} from 'types'
import {IGrid} from 'play/ports/i-grid'

const buildGameConfig = (config: DeepPartial<IGameConfig>): IGameConfig => ({
    grid: uobj.setDefault(config.grid ?? {}, {
        minImageWidth: 50,
        maxImageWidth: 100,

        minImageHeigth: 50,
        maxImageHeigth: 100,

        percentImageWidth: 0.1,
        percentImageHeigth: 0.1
    })
})

export class Dimensions implements IGame {
    width!: number
    height!: number

    canvas: ICanvas = dom.createCanvas(this.id)
    config: IGameConfig
    grid: IGrid

    private x!: X

    constructor(
        public id: string,
        private nickname: string,
        private description: string,
        config: DeepPartial<IGameConfig>
    ) {
        this.config = buildGameConfig(config)
        this.grid = Grid.build(this)
    }

    async setup(): Promise<void> {
        this.x = await core.init(this.nickname, this.description)
        dom.append(this.canvas)
    }

    draw(): void {
        const canvas = this.canvas
        const context = canvas.context
        const element = canvas.element
        context.clearRect(0, 0, element.width, element.height)
        context.fillStyle = 'black'
        context.fillRect(0, 0, element.width, element.height)
    }

    static build(id: string, config: DeepPartial<IGameConfig> = {}) {
        return new Dimensions(id, 'Nickname', 'Description', config)
    }
}
