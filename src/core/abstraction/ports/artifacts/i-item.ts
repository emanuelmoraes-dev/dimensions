import IBaseAbs from '../components/i-base'
import IFightAttrsAbs from '../components/i-fight-attrs'

export default interface IItemAbs {
    base: IBaseAbs
    fightAttrsDamage?: IFightAttrsAbs
    fightAttrsNeeded?: IFightAttrsAbs
}
