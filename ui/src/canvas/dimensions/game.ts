import core from 'canvas/dimensions/core'
import dom from '../ports/operations/dom'
import uobj from 'util/uobj'
import {Grid} from 'canvas/dimensions/grid'
import {X} from 'core/pkg/core'
import {IGame} from 'canvas/ports/i-game'
import {ICanvas} from 'canvas/ports/i-obj'
import {IGameConfig} from 'canvas/ports/i-config'
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

    canvas: ICanvas = dom.createCanvas(this.id)
    config: IGameConfig
    grid!: Grid

    private x!: X
    private lastTime: number = new Date().getTime()
    private countTime = 0
    private images: HTMLImageElement[] = []
    private imagesBase!: HTMLImageElement[]

    readonly maxCountTime = 5000
    readonly colors = ['red', 'green', 'blue', 'yellow', 'orange', 'purple', 'pink', 'brown', 'black']

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
        this.grid = Grid.build(this)
        this.imagesBase = this.colors.map(color => {
            const image = new Image()
            image.src = 'data:image/svg+xml,' +
                `<svg xmlns='http://www.w3.org/2000/svg' width='${this.grid.imageWidth}' height='${this.grid.imageHeight}'>` +
                `<rect width='100%' height='100%' fill='${color}' />` +
                '</svg>'
            return image
        })
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

        // images.push(...images)
        // images.push(...images)

        const imageWidth = this.grid.imageWidth
        const imageHeight = this.grid.imageHeight

        const maxImageSize = Math.max(imageWidth, imageHeight)
        const maxCanvasSize = Math.max(canvasWidth, canvasHeight)
        const maxDeep = Math.floor(maxCanvasSize / maxImageSize) + 2

        // let grid = this.grid
        // const length = Math.floor(images.length * (this.countTime / this.maxCountTime)) + 1
        // for (let i = 0; i < length; i++) {
        //     if (i >= images.length) {
        //         break
        //     }
        //     const image = images[i]
        //     grid.draw(this.canvas, image, imageWidth, imageHeight)
        //     grid = grid.next()
        // }

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
                const imageBaseIndex = Math.floor(Math.random() * this.imagesBase.length)
                image = this.imagesBase[imageBaseIndex]
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
