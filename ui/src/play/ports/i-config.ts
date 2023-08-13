export interface IGameConfig {
    grid: IGridConfig
}

export interface IGridConfig {
    minImageWidth: number
    maxImageWidth: number

    minImageHeigth: number
    maxImageHeigth: number

    percentImageWidth: number
    percentIMageHeigth: number
}
