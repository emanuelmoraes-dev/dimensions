import init, {XCore} from 'assets/wasm/core.js'

export default {
    async init(nickname: string, description: string): Promise<XCore> {
        await init()
        return new XCore(nickname, description)
    }
}
