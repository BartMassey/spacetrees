use crate::*;

type Split<D, T> = Partition<D, QuadTree<D, T>>;
pub struct QuadChildren<D, T>(Partition<D, Split<D, T>>);

pub type QuadTree<D, T> = SpaceTree<T, QuadChildren<D, T>>;

impl<D: Ord, T> Children<T> for QuadChildren<D, T> {
    type Target = [D; 2];

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self> {
        let next = self.0.child(&target[0]);
        next.child(&target[1])
    }
}
