use super::t_map::TMap;

pub trait TUniverse<I, M: TMap<I>> {
    fn current_map(&mut self) -> &mut M;
}
