export type obj = {[k: string]: unknown}

export type IsNot<T, N> = T extends N ? never : T
export type IsPrimitiveObj<T> = T extends obj ? {
    [K in keyof T]: IsNot<T[K], obj>
} : never
export type DeepPartial<T> = (T extends obj ? {
    [K in keyof T]?: T[K] extends IsPrimitiveObj<T[K]> ? Partial<T[K]> : DeepPartial<T[K]>
} : T) | Partial<T>
