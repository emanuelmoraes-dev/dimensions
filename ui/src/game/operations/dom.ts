import { ICanvas, IElement } from 'game/ports/i-obj';

export default {
    append(...elements: IElement[]): void {
        elements.forEach(e => e.parent.appendChild(e.element))
    },
    createCanvas(id: string, width: number, height: number): ICanvas {
        const element = document.createElement('canvas')
        element.id = id
        element.width = width
        element.height = height
        return {
            id,
            width,
            height,
            parent: document.body,
            element: element,
            context: element.getContext('2d') as CanvasRenderingContext2D
        }
    }
}
