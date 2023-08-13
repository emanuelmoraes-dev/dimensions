import {IGame} from './i-game'

export interface IGrid {
    imageWidth: number
    imageHeight: number
    x: number
    y: number
    deep: number

    next(): IGrid
    draw(game:IGame, image: CanvasImageSource): boolean
}
