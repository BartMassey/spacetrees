use crate::*;

pub struct BinChildren<D, T>(Partition<D, [BinTree<D, T>; 2]>);

pub type BinTree<D, T> = SpaceTree<T, BinChildren<D, T>>;

impl<D: Ord, T> Children<T> for BinChildren<D, T> {
    type Target = D;

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self> {
        if *target < self.0.mid {
            &self.0.split[0]
        } else {
            &self.0.split[1]
        }
    }
}
