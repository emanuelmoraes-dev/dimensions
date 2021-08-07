import IBaseAbs from '../components/i-base'
import IBondAbs from '../components/i-bond'
import IPersonalityAbs from '../components/i-personality'
import IFightAttrsAbs from '../components/i-fight-attrs'

export default interface ICharacterAbs {
    base: IBaseAbs
    fightAttrs: IFightAttrsAbs
    fightAbsorbAttrs: IFightAttrsAbs
    personalities: IPersonalityAbs[]
    bonds: IBondAbs[]
}
