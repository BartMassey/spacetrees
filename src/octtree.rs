use crate::*;

type Split0<D, T> = Partition<D, [OctTree<D, T>; 2]>;
type Split1<D, T> = Partition<D, [Split0<D, T>; 2]>;
pub struct OctChildren<D, T>(Partition<D, [Split1<D, T>; 2]>);

pub type OctTree<D, T> = SpaceTree<T, OctChildren<D, T>>;

impl<D: Ord, T> Children<T> for OctChildren<D, T> {
    type Target = [D; 3];

    fn select(&self, target: &Self::Target) -> &SpaceTree<T, Self> {
        let next = if target[0] < self.0.mid {
            &self.0.split[0]
        } else {
            &self.0.split[1]
        };
        let next = if target[1] < next.mid {
            &next.split[0]
        } else {
            &next.split[1]
        };
        if target[2] < next.mid {
            &next.split[0]
        } else {
            &next.split[1]
        }
    }
}
