use crate::*;

type Split<D, T> = Partition<D, [QuadTree<D, T>; 2]>;
pub struct QuadChildren<D, T>(Partition<D, [Split<D, T>; 2]>);

pub type QuadTree<D, T> = SpaceTree<T, QuadChildren<D, T>>;

impl<D: Ord, T> Children<T> for QuadChildren<D, T> {
    type Target = [D; 2];

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self> {
        let next = if target[0] < self.0.mid {
            &self.0.split[0]
        } else {
            &self.0.split[1]
        };
        if target[1] < next.mid {
            &next.split[0]
        } else {
            &next.split[1]
        }
    }
}
