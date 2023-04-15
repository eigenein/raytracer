use crate::math::aabb::{Aabb, Bounded};

/// Bounding volume hierarchy.
pub enum Bvh<'a, T> {
    Empty,
    Leaf(Leaf<'a, T>),
    Node(Box<Node<'a, T>>),
}

pub struct Node<'a, T> {
    aabb: Aabb,
    left: Bvh<'a, T>,
    right: Bvh<'a, T>,
}

pub struct Leaf<'a, T> {
    aabb: Aabb,
    surfaces: &'a [T],
}

impl<'a, T: Bounded> Bvh<'a, T> {
    pub fn new(surfaces: &'a mut [T]) -> Self {
        if surfaces.is_empty() {
            return Self::Empty;
        }

        // Find out the AABB that encompasses all the surfaces.
        let aabb = surfaces[1..]
            .iter()
            .map(|surface| surface.aabb())
            .fold(surfaces[0].aabb(), |accumulator, aabb| accumulator | aabb);

        if surfaces.len() <= 2 {
            // Don't bother splitting too few surfaces.
            return Self::Leaf(Leaf { aabb, surfaces });
        }

        // Split by maximal dimension:
        let size = aabb.size();
        let key = if size.x > size.y && size.x > size.z {
            |surface: &T| surface.aabb().center().x
        } else if size.y > size.x && size.y > size.z {
            |surface: &T| surface.aabb().center().y
        } else {
            |surface: &T| surface.aabb().center().z
        };
        surfaces.sort_unstable_by(|lhs, rhs| key(lhs).total_cmp(&key(rhs)));
        let (left, right) = surfaces.split_at_mut(surfaces.len() / 2);
        Self::Node(Box::new(Node {
            aabb,
            left: Bvh::new(left),
            right: Bvh::new(right),
        }))
    }
}
