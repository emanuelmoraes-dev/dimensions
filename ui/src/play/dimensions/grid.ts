import umath from 'util/umath'
import {IGrid} from 'play/ports/i-grid'
import {IGame} from 'play/ports/i-game'

enum Direction {
    Left,
    Top,
    Right,
    Down
}

const getNextCoord = (x: number, y: number, ...directions: Direction[]): {x: number, y: number} => {
    directions.forEach(d => {
        switch(d) {
        case Direction.Left:
            x--
            break
        case Direction.Top:
            y++
            break
        case Direction.Right:
            x++
            break
        case Direction.Down:
            y--
            break
        }
    })
    return {x, y}
}

const getNextDirection = (d: Direction): Direction => {
    return ((d + 1) % Object.keys(Direction).length) as Direction
}

export class Grid implements IGrid {
    constructor(
        public imageWidth: number,
        public imageHeight: number,
        public x: number,
        public y: number,
        public deep: number,
        private direction: Direction,
        private position: number
    ) {}

    next(): IGrid {
        if (this.position === 0) {
            const {x, y} = getNextCoord(this.x, this.y, Direction.Right, Direction.Down)
            const position = 1
            const deep = 2
            return new Grid(this.imageWidth, this.imageHeight, x, y, deep, Direction.Left, position)
        }

        let direction: Direction
        if (this.position % this.deep === 0) {
            direction = getNextDirection(this.direction)
        } else {
            direction = this.direction
        }

        let x: number
        let y: number
        let position: number
        let deep: number
        if (direction === Direction.Left && direction !== this.direction) {
            const {x: nx, y: ny} = getNextCoord(this.x, this.y, Direction.Down, Direction.Down)
            x = nx
            y = ny
            deep = this.deep + 1
            position = 1
        } else {
            const {x: nx, y: ny} = getNextCoord(this.x, this.y, this.direction)
            x = nx
            y = ny
            deep = this.deep
            position = this.position + 1
        }

        return new Grid(this.imageWidth, this.imageHeight, x, y, deep, direction, position)
    }

    draw(game: IGame, image: CanvasImageSource): boolean {
        return false
    }

    static build(game: IGame): Grid {
        const gameWidth = game.width
        const gameHeigth = game.height
        const config = game.config.grid
        const imageWidth = umath.interval(config.minImageWidth, config.maxImageWidth, gameWidth * config.percentImageWidth)
        const imageHeight = umath.interval(config.minImageHeigth, config.maxImageHeigth, gameHeigth * config.percentImageHeigth)
        const x = 0
        const y = 0
        const direction = Direction.Left
        const position = 0
        const deep = 1
        return new Grid(imageWidth, imageHeight, x, y, deep, direction, position)
    }
}
