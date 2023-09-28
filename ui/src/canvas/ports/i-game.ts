import {ICanvas, Iid} from 'canvas/ports/i-obj.ts'
import {IGameConfig} from 'canvas/ports/i-config.ts'
import {IGrid} from 'canvas/ports/i-grid.ts'

export interface IGame extends Iid {
    width: number
    height: number
    canvas: ICanvas
    config: IGameConfig
    grid: IGrid

    setup(): Promise<void>
    loop(): void
}
