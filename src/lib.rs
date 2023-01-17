mod bintree;
mod quadtree;
mod octtree;

pub use bintree::*;
pub use quadtree::*;
pub use octtree::*;

use std::cmp::Ord;

pub struct Partition<D, C> {
    pub(crate) mid: D,
    pub(crate) split: C,
}

pub enum SpaceTree<T, C> {
    Leaf(Vec<T>),
    SubTree(Box<C>),
}

pub trait Children<T>: Sized {
    type Target;

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self>;
}

impl<T, C: Children<T>> SpaceTree<T, C> {
    pub fn find(&self, target: &C::Target) -> &[T] {
        let mut cur = self;
        loop {
            match cur {
                SpaceTree::SubTree(ref p) => cur = p.select(target),
                SpaceTree::Leaf(ref v) => return v.as_ref(),
            }
        }
    }
}
