import core from 'canvas/dimensions/core.ts'
import dom from 'canvas/ports/operations/dom.ts'
import uobj from 'util/uobj.ts'
import {Grid} from 'canvas/dimensions/grid.ts'
import {XCore, XDebug, XMap} from 'assets/wasm/core.js'
import {IGame} from 'canvas/ports/i-game.ts'
import {ICanvas} from 'canvas/ports/i-obj.ts'
import {IGameConfig} from 'canvas/ports/i-config.ts'
import {DeepPartial} from 'types'

const buildGameConfig = (config: DeepPartial<IGameConfig>): IGameConfig => ({
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
    private lastTime: number = new Date().getTime()
    private countTime = 0
    private images: HTMLImageElement[] = []

    readonly maxCountTime = 5000
    readonly colors = ['red', 'green', 'blue', 'orange', 'purple', 'brown', 'black']

    constructor(
        public id: string,
        private nickname: string,
        private description: string,
        config: DeepPartial<IGameConfig>
    ) {
        this.config = buildGameConfig(config)
        this.canvas = dom.createCanvas(this.id)
    }

    async setup(): Promise<void> {
        this.core = await core.init(this.nickname, this.description)
        XDebug.show_character(this.core)
        dom.append(this.canvas)
        this.grid = Grid.build(this)
    }

    draw(): void {
        const diffTime = new Date().getTime() - this.lastTime
        this.lastTime = new Date().getTime()
        this.countTime += diffTime
        if (this.countTime > this.maxCountTime) {
            this.countTime = 0
        }

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

        let imagesMaxLength: number

        if (this.images.length > 0) {
            imagesMaxLength = Math.floor(this.images.length * (this.countTime / this.maxCountTime)) * 1.3
        } else {
            imagesMaxLength = Number.MAX_SAFE_INTEGER
        }

        let drawn = false
        let grid = this.grid
        let lastDeep = Math.min(grid.deepWidth, grid.deepHeight)
        let imageIndex = 0
        while (Math.min(grid.deepWidth, grid.deepHeight) <= maxDeep && imageIndex < imagesMaxLength) {
            let image: HTMLImageElement

            if (imageIndex < this.images.length) {
                image = this.images[imageIndex]
            } else {
                const ximage = XMap.getLocation(this.core, grid.x, grid.y, grid.imageWidth, grid.imageHeight)

                if (!ximage) {
                    throw new Error('moveLocation failed')
                }

                const data = ximage.data()
                const blob = new Blob([data], {type: 'image/png'})
                const imageUrl = URL.createObjectURL(blob)
                image = new Image()
                image.src = imageUrl
            }

            if (grid.draw(this.canvas, image, imageWidth, imageHeight)) {
                drawn = true
                if (imageIndex >= this.images.length) {
                    this.images.push(image)
                }
                imageIndex++
            }

            grid = grid.next()

            if (Math.min(grid.deepWidth, grid.deepHeight) > lastDeep) {
                lastDeep = Math.min(grid.deepWidth, grid.deepHeight)

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
