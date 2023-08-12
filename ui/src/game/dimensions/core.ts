import init, { Dimensions, run } from 'core/pkg/core.js'

export default {
    async init(nickname: string, description: string): Promise<Dimensions> {
        await init()
        const dimensions = new Dimensions(nickname, description)
        run(dimensions)
        return dimensions
    }
}
