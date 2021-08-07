import {IComposeType} from '../../../types'

export default interface IFightAttrsAbs {
    vitality: number
    strength: number
    dexterity: number
    stamina: number
    weight: number
    endurance: number
    speed: number
    intelligence: number
    luck: number
    fly: IComposeType<number,
        'speed' |
        'duration' |
        'altitude'>,
    resistance: IComposeType<number,
        'physics' |
        'magic' |
        'fire' |
        'water' |
        'electricity' |
        'air'>
}
