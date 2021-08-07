export type IComposeType<T, Keys extends string> = {
    // eslint-disable-next-line no-unused-vars
    [k in Keys]: T
}
