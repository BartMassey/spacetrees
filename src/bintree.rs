use crate::*;

type BinChildren<D, T> = Partition<D, [BinTree<D, T>; 2]>;

pub enum BinTree<D, T> {
    Leaf(Vec<T>),
    SubTree(Box<BinChildren<D, T>>),
}

impl<D: Ord, T> BinTree<D, T> {
    pub fn find(&self, target: &D) -> &[T] {
        let mut cur = self;
        loop {
            match cur {
                BinTree::SubTree(ref p) => cur = p.select(target),
                BinTree::Leaf(ref v) => return v.as_ref(),
            }
        }
    }
}

impl<D: Ord, T> BinChildren<D, T> {
    fn select(&self, target: &D) -> &BinTree<D, T> {
        if *target < self.mid {
            &self.split[0]
        } else {
            &self.split[1]
        }
    }
}
