import core from 'canvas/dimensions/core.ts'
import dom from 'canvas/ports/operations/dom.ts'
import uobj from 'util/u-obj.ts'
import {Grid} from 'canvas/dimensions/grid.ts'
import {XCore, XDebug} from 'assets/wasm/core.js'
import {IGame} from 'canvas/ports/i-game.ts'
import {ICanvas} from 'canvas/ports/i-obj.ts'
import {IGameConfig} from 'canvas/ports/i-config.ts'
import {DeepPartial} from 'types'
import {Animation} from 'util/u-animation.ts'
import {MapImages} from 'canvas/dimensions/map-images.ts'

const buildParent = (parent: string | HTMLElement): HTMLElement => {
    if (typeof parent === 'string') {
        const element = document.getElementById(parent)
        if (!element) {
            throw new Error(`Element with id "${parent}" not found`)
        }
        return element
    }
    return parent
}

const buildGameConfig = (config: DeepPartial<IGameConfig>): IGameConfig => ({
    parent: config.parent || document.body,
    grid: uobj.setDefault(config.grid ?? {}, {
        minImageWidth: 70,
        maxImageWidth: 100,

        minImageHeigth: 70,
        maxImageHeigth: 100,

        percentImageSize: 0.1,
        useMaxPercent: true,
        aspectRatio: 1
    })
})

export class Dimensions implements IGame {
    width!: number
    height!: number

    canvas: ICanvas
    config: IGameConfig
    grid!: Grid

    private core!: XCore
    private animation!: Animation
    private map!: MapImages

    constructor(
        public id: string,
        private nickname: string,
        private description: string,
        config: DeepPartial<IGameConfig>
    ) {
        this.config = buildGameConfig(config)
        this.canvas = dom.createCanvas(buildParent(this.config.parent), this.id)
        this.animation = new Animation(5000)
    }

    async setup(): Promise<void> {
        this.core = await core.init(this.nickname, this.description)
        dom.append(this.canvas)

        this.map = new MapImages(this.core)
        this.grid = Grid.build(this.canvas, this.config.grid)

        XDebug.showCharacter(this.core)

        // setInterval(() => {
        //     this.map.clear()
        // }, 3000);
    }

    draw(): void {
        this.animation.tick()

        const canvas = this.canvas
        const context = canvas.context
        const canvasWidth = canvas.element.width
        const canvasHeight = canvas.element.height
        context.clearRect(0, 0, canvasWidth, canvasHeight)

        const imageWidth = this.grid.imageWidth
        const imageHeight = this.grid.imageHeight

        const maxImageSize = Math.max(imageWidth, imageHeight)
        const maxCanvasSize = Math.max(canvasWidth, canvasHeight)
        const maxDeep = Math.floor(maxCanvasSize / maxImageSize) + 2

        let drawn = false
        let grid = this.grid
        let lastDeep = grid.minDeep
        while (grid.minDeep <= maxDeep) {
            const image = this.map.getImage(grid)

            if (grid.draw(this.canvas, image, imageWidth, imageHeight)) {
                drawn = true
                this.map.mark(grid, {drawn: true})
            } else {
                this.map.mark(grid, {drawn: false})
            }

            grid = grid.next()

            if (grid.minDeep > lastDeep) {
                lastDeep = grid.minDeep

                if (!drawn) {
                    break
                }

                drawn = false
            }
        }
    }

    static build(id: string, config: DeepPartial<IGameConfig> = {}) {
        return new Dimensions(id, 'Nickname', 'Description', config)
    }
}
