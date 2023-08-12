import {ICanvas, Iid} from 'play/ports/i-obj'

export interface IGame extends Iid {
    width: number;
    height: number;
    canvas: ICanvas;

    config(): Promise<void>;
    draw(): void;
}
