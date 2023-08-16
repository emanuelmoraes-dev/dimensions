import {ICanvas} from './i-obj'

export interface IGrid {
    imageWidth: number
    imageHeight: number
    x: number
    y: number

    next(): IGrid
    draw(canvas: ICanvas, image: CanvasImageSource, imageWidth: number, imageHeight: number): boolean
}
