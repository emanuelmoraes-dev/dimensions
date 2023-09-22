export default {
    async onload(callback: () => Promise<unknown>) {
        await new Promise<void>(resolve => {
            if (document.readyState === 'complete' || document.readyState === 'interactive') {
                resolve()
            } else {
                document.addEventListener('DOMContentLoaded', () => resolve())
            }
        })
        return callback()
    }
}
