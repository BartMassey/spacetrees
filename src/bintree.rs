use crate::*;

pub struct BinChildren<D, T>(Partition<D, BinTree<D, T>>);

pub type BinTree<D, T> = SpaceTree<T, BinChildren<D, T>>;

impl<D: Ord, T> Children<T> for BinChildren<D, T> {
    type Target = D;

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self> {
        self.0.child(target)
    }
}
