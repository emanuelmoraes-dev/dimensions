import {ICanvas, Iid} from 'play/ports/i-obj'
import {IGameConfig} from 'play/ports/i-config'

export interface IGame extends Iid {
    width: number
    height: number
    canvas: ICanvas
    config: IGameConfig

    setup(): Promise<void>
    draw(): void
}
