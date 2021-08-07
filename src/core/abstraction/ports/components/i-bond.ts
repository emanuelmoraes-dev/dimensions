export interface IBondTypeAbs {
    name: string
    description: string
}

export default interface IBondAbs {
    name: string
    type: IBondTypeAbs
    description: string
}
