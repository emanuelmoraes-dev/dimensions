import {ICanvas, Iid} from 'canvas/ports/i-obj'
import {IGameConfig} from 'canvas/ports/i-config'
import {IGrid} from 'canvas/ports/i-grid'

export interface IGame extends Iid {
    width: number
    height: number
    canvas: ICanvas
    config: IGameConfig
    grid: IGrid

    setup(): Promise<void>
    draw(): void
}
