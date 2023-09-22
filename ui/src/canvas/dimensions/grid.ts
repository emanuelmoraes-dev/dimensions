import umath from 'util/u-math.ts'
import {IGrid} from 'canvas/ports/i-grid.ts'
import {IGame} from 'canvas/ports/i-game.ts'
import {ICanvas} from 'canvas/ports/i-obj.ts'

enum DirectionEnum {
    Left,
    Top,
    Right,
    Down
}

const getNextCoord = (x: number, y: number, ...directions: DirectionEnum[]): {x: number, y: number} => {
    directions.forEach(d => {
        switch(d) {
        case DirectionEnum.Left:
            x--
            break
        case DirectionEnum.Top:
            y--
            break
        case DirectionEnum.Right:
            x++
            break
        case DirectionEnum.Down:
            y++
            break
        }
    })
    return {x, y}
}

const getNextDirection = (d: DirectionEnum): DirectionEnum => {
    switch(d) {
    case DirectionEnum.Left: return DirectionEnum.Top
    case DirectionEnum.Top: return DirectionEnum.Right
    case DirectionEnum.Right: return DirectionEnum.Down
    case DirectionEnum.Down: return DirectionEnum.Left
    }
}

const isVertical = (d: DirectionEnum): boolean => {
    switch(d) {
    case DirectionEnum.Left:
    case DirectionEnum.Right:
        return true
    case DirectionEnum.Top:
    case DirectionEnum.Down:
        return false
    }
}

const isHorizontal = (d: DirectionEnum): boolean => !isVertical(d)

export class Grid implements IGrid {
    constructor(
        public imageWidth: number,
        public imageHeight: number,
        public x: number,
        public y: number,
        public positionWidth: number,
        public positionHeight: number,
        public deepWidth: number,
        public deepHeight: number,
        public direction: DirectionEnum
    ) {}

    next(): Grid {
        let nextPositionWidth = this.positionWidth
        let nextPositionHeight = this.positionHeight
        let nextDeepWidth = this.deepWidth
        let nextDeepHeight = this.deepHeight
        let nextDirection = this.direction

        if (isHorizontal(this.direction)) {
            nextPositionWidth++
            if (nextPositionWidth > this.deepWidth) {
                nextPositionWidth = 1
                nextPositionHeight++
                nextDeepWidth = this.deepWidth + 1
                nextDirection = getNextDirection(this.direction)
            }
        } else {
            nextPositionHeight++
            if (nextPositionHeight > this.deepHeight) {
                nextPositionHeight = 1
                nextPositionWidth++
                nextDeepHeight = this.deepHeight + 1
                nextDirection = getNextDirection(this.direction)
            }
        }

        const {x: nx, y: ny} = getNextCoord(this.x, this.y, nextDirection)

        return new Grid(this.imageWidth, this.imageHeight, nx, ny, nextPositionWidth, nextPositionHeight, nextDeepWidth, nextDeepHeight, nextDirection)
    }

    draw(canvas: ICanvas, image: CanvasImageSource, imageWidth: number, imageHeight: number): boolean {
        const canvasWidth = canvas.element.width
        const canvasHeight = canvas.element.height

        const canvasCenterWidth = canvasWidth / 2
        const canvasCenterHeight = canvasHeight / 2

        const imageCenterWidth = this.imageWidth / 2
        const imageCenterHeight = this.imageHeight / 2

        const imageCenterX = canvasCenterWidth + (this.x * this.imageWidth)
        const imageCenterY = canvasCenterHeight + (this.y * this.imageHeight)

        const odx = imageCenterX - imageCenterWidth
        const ody = imageCenterY - imageCenterHeight

        const dx = Math.max(0, odx)
        const dy = Math.max(0, ody)

        if (dx > canvasWidth || dy > canvasHeight) {
            return false
        }

        const dxDiff = dx - odx
        const dyDiff = dy - ody

        const sx = dxDiff / this.imageWidth * imageWidth
        const sy = dyDiff / this.imageHeight * imageHeight

        const sWidth = imageWidth - sx
        const sHeight = imageHeight - sy

        if (sWidth <= 0 || sHeight <= 0) {
            return false
        }

        const dWidth = this.imageWidth - dxDiff
        const dHeight = this.imageHeight - dyDiff

        if (dWidth <= 0 || dHeight <= 0) {
            return false
        }

        canvas.context.drawImage(image, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight)
        return true
    }

    static build(game: IGame): Grid {
        const config = game.config.grid
        const canvas = game.canvas
        const gameWidth = canvas.element.width
        const gameHeigth = canvas.element.height
        const gameMaxSize = Math.max(gameWidth, gameHeigth)
        const gameMinSize = Math.min(gameWidth, gameHeigth)
        const size = config.useMaxPercent ? gameMaxSize : gameMinSize
        const aspectRationWidth = Math.min(1, config.aspectRatio)
        const aspectRationHeight = Math.min(1, 1 / config.aspectRatio)
        const imageWidth = umath.interval(config.minImageWidth, config.maxImageWidth, size * config.percentImageSize * aspectRationWidth)
        const imageHeight = umath.interval(config.minImageHeigth, config.maxImageHeigth, size * config.percentImageSize * aspectRationHeight)
        const x = 0
        const y = 0
        const positionWidth = 1
        const positionHeight = 1
        const deepWidth = 2
        const deepHeight = 2
        const direction = DirectionEnum.Left
        return new Grid(imageWidth, imageHeight, x, y, positionWidth, positionHeight, deepWidth, deepHeight, direction)
    }
}
