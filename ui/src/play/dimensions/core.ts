import init, {X, xrun} from 'core/pkg/core.js'

export default {
    async init(nickname: string, description: string): Promise<X> {
        await init()
        const dimensions = new X(nickname, description)
        xrun(dimensions)
        return dimensions
    }
}
