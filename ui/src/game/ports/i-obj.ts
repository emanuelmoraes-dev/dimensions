export interface IObj {
    id?: string;
}

export interface Iid {
    id: string;
}

export interface IElement extends Iid {
    parent: HTMLElement;
    element: HTMLElement;
}

export interface ICanvas extends IElement {
    context: CanvasRenderingContext2D,
    width: number,
    height: number
}
