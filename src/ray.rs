use std::borrow::Cow;
use std::ops::Index;

use glam::DVec3;
use smallvec::SmallVec;

type RefractiveIndexes = SmallVec<[f64; 4]>;

pub struct Ray<'a> {
    pub origin: DVec3,

    pub direction: DVec3,

    /// Stack of medium refractive indexes.
    ///
    /// The top element is the current medium's index.
    ///
    /// When the ray enters a new medium, the new index gets pushed onto stack.
    /// When the ray leaves the medium, the former index gets popped from the stack.
    pub refractive_indexes: Cow<'a, RefractiveIndexes>,
}

impl<'a> Ray<'a> {
    const DEFAULT_REFRACTIVE_INDEXES: Cow<'static, RefractiveIndexes> =
        Cow::Owned(SmallVec::new_const());

    #[inline]
    pub fn new(
        origin: DVec3,
        direction: DVec3,
        refractive_indexes: Cow<'a, RefractiveIndexes>,
    ) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            refractive_indexes,
        }
    }

    #[inline]
    pub fn by_two_points(from: DVec3, to: DVec3) -> Self {
        Self::new(from, to - from, Self::DEFAULT_REFRACTIVE_INDEXES)
    }

    #[inline]
    pub fn at(&self, distance: f64) -> DVec3 {
        self.origin + self.direction * distance
    }

    #[inline]
    pub fn current_refractive_index(&self) -> Option<f64> {
        self.refractive_indexes.last().copied()
    }

    /// Get refractive index of the medium one surface across the current medium.
    #[inline]
    pub fn outer_refractive_index(&self) -> Option<f64> {
        let length = self.refractive_indexes.len();
        if length >= 2 {
            Some(*self.refractive_indexes.index(length - 2))
        } else {
            None
        }
    }
}
