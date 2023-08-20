import init, {X, x_show_character} from 'core/pkg/core.js'

export default {
    async init(nickname: string, description: string): Promise<X> {
        await init()
        const x = new X(nickname, description)
        x_show_character(x)
        return x
    }
}
