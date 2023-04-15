use std::fmt::{Debug, Formatter};
use std::ops::Range;

use fastrand::Rng;

use crate::math::aabb::{Aabb, Bounded};
use crate::math::hit::{Hit, Hittable};
use crate::math::ray::Ray;

/// Bounding volume hierarchy.
pub enum Bvh<'a, T> {
    Empty,
    Leaf(&'a [T]),
    Node(Box<Node<'a, T>>),
}

#[derive(Debug)]
pub struct Node<'a, T> {
    aabb: Aabb,
    left: Bvh<'a, T>,
    right: Bvh<'a, T>,
}

impl<'a, T: Bounded> Bvh<'a, T> {
    pub fn new(surfaces: &'a mut [T], max_leaf_size: usize) -> Self {
        if surfaces.is_empty() {
            return Self::Empty;
        }

        // Find out the AABB that encompasses all the surfaces.
        let aabb = surfaces[1..]
            .iter()
            .map(|surface| surface.aabb())
            .fold(surfaces[0].aabb(), |accumulator, aabb| accumulator | aabb);

        if surfaces.len() <= max_leaf_size {
            return Self::Leaf(surfaces);
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
            left: Bvh::new(left, max_leaf_size),
            right: Bvh::new(right, max_leaf_size),
        }))
    }
}

impl<'a, T: Hittable> Hittable for Bvh<'a, T> {
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>, rng: &Rng) -> Option<Hit> {
        match self {
            Self::Empty => None,

            // For a leaf run the good old sequential search.
            Self::Leaf(surfaces) => surfaces
                .iter()
                .filter_map(|surface| surface.hit(by_ray, distance_range, rng))
                .min_by(|hit_1, hit_2| hit_1.distance.total_cmp(&hit_2.distance)),

            // For a node, delegate the checks to the child nodes.
            Self::Node(node) => {
                if node.aabb.hit(by_ray, distance_range).is_some() {
                    let left_hit = node.left.hit(by_ray, distance_range, rng);
                    let right_hit = node.right.hit(by_ray, distance_range, rng);
                    if left_hit < right_hit {
                        left_hit
                    } else {
                        right_hit
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl<'a, T> Debug for Bvh<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Leaf(leaf) => write!(f, "Leaf[{}]", leaf.len()),
            Self::Node(node) => write!(
                f,
                "Node[{}, {}](left: {:?}, right: {:?})",
                node.aabb.min_point, node.aabb.max_point, node.left, node.right
            ),
        }
    }
}
