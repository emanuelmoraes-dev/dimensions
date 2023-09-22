export class Animation {
    private countTime: number = 0
    private lastTime: number = new Date().getTime()

    constructor (
        public duration: number
    ) {}

    get percent(): number {
        return this.countTime / this.duration
    }

    tick(): void {
        const diffTime = new Date().getTime() - this.lastTime
        this.lastTime = new Date().getTime()
        this.countTime += diffTime
        if (this.countTime > this.duration) {
            this.countTime = 0
        }
    }
}
