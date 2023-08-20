export interface IGameConfig {
    grid: IGridConfig
}

export interface IGridConfig {
    minImageWidth: number
    maxImageWidth: number

    minImageHeigth: number
    maxImageHeigth: number

    percentImageSize: number
    useMaxPercent: boolean

    aspectRatio: number
}
