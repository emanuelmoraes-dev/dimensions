import {XCore, XMap} from 'assets/wasm/core.js'
import {Grid} from 'canvas/dimensions/grid.ts'
import {obj} from 'types'

type MapCoords = {
    [x: number]: {
        [y: number]: number
    }
}

type MapDrawn = {
    [index: number]: State | undefined
}

export interface State extends obj {
    drawn?: boolean
}

export class MapImages {
    private images: HTMLImageElement[] = []
    private mapping: MapCoords = {}
    private mappingState: MapDrawn = {}

    constructor(
        private core: XCore
    ) {}

    length(state?: State): number {
        if (!state) {
            return this.images.length
        }

        return this.images.reduce((acc, _, index) => {
            const storedState = this.mappingState[index]
            if (storedState && this.matchState(storedState, state)) {
                acc++
            }
            return acc
        }, 0)
    }

    getImage(grid: Grid): HTMLImageElement {
        if (!(grid.x in this.mapping)) {
            this.mapping[grid.x] = {}
        }

        let index = this.mapping[grid.x][grid.y] ?? -1

        if (index < 0 || index >= this.images.length) {
            const image = this.buildImage(grid)
            index = this.images.length
            this.mapping[grid.x][grid.y] = index
            this.images.push(image)
        }

        return this.images[index]
    }

    mark(grid: Grid, state: State): void {
        if (!(grid.x in this.mapping)) {
            return
        }

        const index = this.mapping[grid.x][grid.y] ?? -1

        if (index < 0 || index >= this.images.length) {
            return
        }

        this.mappingState[index] = state
    }

    clear(): void {
        const clearImages = this.images
        this.images = []
        this.mapping = {}
        this.mappingState = {}

        for (const image of clearImages) {
            URL.revokeObjectURL(image.src)
        }

        XMap.clear(this.core)
    }

    private buildImage(grid: Grid) {
        const ximage = XMap.getLocation(this.core, grid.x, grid.y, grid.imageWidth, grid.imageHeight)

        if (!ximage) {
            throw new Error('getLocation failed')
        }

        const data = ximage.data()
        const blob = new Blob([data], {type: 'image/png'})
        const imageUrl = URL.createObjectURL(blob)
        const image = new Image()
        image.src = imageUrl
        return image
    }

    private matchState(stored: State, target: State): boolean {
        const keys = Object.keys(target)

        for (const key of keys) {
            const storedValue = (stored as obj)[key]
            const targetValue = (target as obj)[key]

            if (targetValue === undefined) {
                continue
            }

            if (storedValue !== targetValue) {
                return false
            }
        }

        return true
    }
}
