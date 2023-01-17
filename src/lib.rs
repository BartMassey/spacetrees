mod bintree;
mod quadtree;
mod octtree;

pub use bintree::*;
pub use quadtree::*;
pub use octtree::*;

use std::cmp::Ord;

pub struct Partition<D, C> {
    pub(crate) mid: D,
    pub(crate) split: [C; 2],
}

impl<D: Ord, C> Partition<D, C> {
    pub(crate) fn child(&self, target: &D) -> &C {
        if *target < self.mid {
            &self.split[0]
        } else {
            &self.split[1]
        }
    }

    pub(crate) fn child_mut(&mut self, target: &D) -> &mut C {
        if *target < self.mid {
            &mut self.split[0]
        } else {
            &mut self.split[1]
        }
    }
}

pub enum SpaceTree<T, C> {
    Leaf(Vec<T>),
    SubTree(Box<C>),
}

pub trait Children<T>: Sized {
    type Target;

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self>;
    fn select_mut(&mut self, target: &Self::Target) -> &mut SpaceTree<T, Self>;
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

    pub fn find_mut(&mut self, target: &C::Target) -> &mut Vec<T> {
        let mut cur = self;
        loop {
            match cur {
                SpaceTree::SubTree(ref mut p) => cur = p.select_mut(target),
                SpaceTree::Leaf(ref mut v) => return v,
            }
        }
    }
}
