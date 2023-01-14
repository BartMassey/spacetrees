mod bintree;
mod quadtree;

pub use bintree::*;
pub use quadtree::*;

use std::cmp::Ord;

pub struct Partition<D, C> {
    pub(crate) mid: D,
    pub(crate) split: C,
}
