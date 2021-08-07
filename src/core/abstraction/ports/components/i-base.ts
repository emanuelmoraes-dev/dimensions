export interface IRoleTypeAbs {
    name: string
    description: string
}

export interface IRoleAbs {
    type: IRoleTypeAbs
}

export interface IBaseTypeAbs {
    name: string
    description: string
    roles: IRoleAbs[]
}

export default interface IBaseAbs {
    name: string
    type: IBaseTypeAbs[]
    roles: IRoleAbs[]
}
