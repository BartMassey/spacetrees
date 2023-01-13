use std::cmp::Ord;

pub struct Partition<D, C> {
    mid: D,
    split: C,
}

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

type QuadChildren<D, T> = Partition<D, [Partition<D, [QuadTree<D, T>; 2]>; 2]>;

pub enum QuadTree<D, T> {
    Leaf(Vec<T>),
    SubTree(Box<QuadChildren<D, T>>),
}

impl<D: Ord, T> QuadTree<D, T> {
    pub fn find(&self, target: &[D; 2]) -> &[T] {
        let mut cur = self;
        loop {
            match cur {
                QuadTree::SubTree(p) => cur = (**p).select(target),
                QuadTree::Leaf(ref v) => return v.as_ref(),
            }
        }
    }
}

impl<D: Ord, T> QuadChildren<D, T> {
    fn select(&self, target: &[D; 2]) -> &QuadTree<D, T> {
        let next = if target[0] < self.mid {
            &self.split[0]
        } else {
            &self.split[1]
        };
        if target[1] < next.mid {
            &next.split[0]
        } else {
            &next.split[1]
        }
    }
}
