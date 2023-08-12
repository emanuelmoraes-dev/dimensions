import {Iid} from './i-obj'

export interface IGame extends Iid {
    width: number;
    heigth: number;

    config(): Promise<void>;
    draw(): void;
}
