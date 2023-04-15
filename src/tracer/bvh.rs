use std::fmt::{Debug, Formatter};
use std::ops::Range;

use fastrand::Rng;

use crate::math::aabb::{Aabb, Bounded};
use crate::math::hit::{Hit, Hittable};
use crate::math::ray::Ray;
use crate::math::vec3::Vec3;

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

        if surfaces.len() <= max_leaf_size {
            return Self::Leaf(surfaces);
        }

        // Find out the AABB that encompasses all the surfaces.
        let aabb = surfaces[1..]
            .iter()
            .map(|surface| surface.aabb())
            .fold(surfaces[0].aabb(), |accumulator, aabb| accumulator | aabb);
        let size = aabb.size();
        let center = aabb.center();

        // Split by maximal dimension:
        let key = if size.x > size.y && size.x > size.z {
            |vec: Vec3| vec.x
        } else if size.y > size.x && size.y > size.z {
            |vec: Vec3| vec.y
        } else {
            |vec: Vec3| vec.z
        };

        // Sort by the maximal dimension:
        surfaces.sort_unstable_by(|lhs, rhs| {
            key(lhs.aabb().center()).total_cmp(&key(rhs.aabb().center()))
        });

        // Split by the mean:
        let (left, right) = surfaces.split_at_mut(
            surfaces.partition_point(|surface| key(surface.aabb().center()) < key(center)),
        );

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
                    match (left_hit, right_hit) {
                        (Some(left_hit), Some(right_hit)) => {
                            if left_hit.distance < right_hit.distance {
                                Some(left_hit)
                            } else {
                                Some(right_hit)
                            }
                        }
                        (left_hit @ Some(_), None) => left_hit,
                        (_, right_hit) => right_hit,
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
