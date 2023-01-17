use crate::*;

type Split0<D, T> = Partition<D, OctTree<D, T>>;
type Split1<D, T> = Partition<D, Split0<D, T>>;
pub struct OctChildren<D, T>(Partition<D, Split1<D, T>>);

pub type OctTree<D, T> = SpaceTree<T, OctChildren<D, T>>;

impl<D: Ord, T> Children<T> for OctChildren<D, T> {
    type Target = [D; 3];

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self> {
        let next = self.0.child(&target[0]);
        let next = next.child(&target[1]);
        next.child(&target[2])
    }

    fn select_mut(&mut self, target: &Self::Target) -> &mut SpaceTree<T, Self> {
        let next = self.0.child_mut(&target[0]);
        let next = next.child_mut(&target[1]);
        next.child_mut(&target[2])
    }
}
