import {ICanvas, Iid} from 'play/ports/i-obj'
import {IGameConfig} from 'play/ports/i-config'
import {IGrid} from 'play/ports/i-grid'

export interface IGame extends Iid {
    width: number
    height: number
    canvas: ICanvas
    config: IGameConfig
    grid: IGrid

    setup(): Promise<void>
    draw(): void
}
