import {ICanvas} from 'canvas/ports/i-obj.ts'

export interface IGrid {
    imageWidth: number
    imageHeight: number
    x: number
    y: number

    next(): IGrid
    draw(canvas: ICanvas, image: CanvasImageSource, imageWidth: number, imageHeight: number): boolean
}
