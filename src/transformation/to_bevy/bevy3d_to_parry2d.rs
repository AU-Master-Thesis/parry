use crate::{
    math::{Real, Vector},
    shape,
};
#[cfg(feature = "bevy")]
use bevy::math::primitives;

// implement conversions form parry2d shape::Cuboid to bevy::math::primitives::Cuboid
#[cfg(feature = "dim2")]
impl Into<shape::Cuboid> for primitives::Cuboid {
    fn into(self) -> shape::Cuboid {
        let (x, y) = (self.half_size.x as Real, self.half_size.z as Real);
        shape::Cuboid::new(Vector::new(x, y))
    }
}

// implement conversions form parry3d shape::Cuboid to bevy::math::primitives::Cuboid
#[cfg(feature = "dim3")]
impl Into<shape::Cuboid> for primitives::Cuboid {
    fn into(self) -> shape::Cuboid {
        let (x, y, z) = (
            self.half_size.x as Real,
            self.half_size.y as Real,
            self.half_size.z as Real,
        );
        shape::Cuboid::new(Vector::new(x, y, z))
    }
}

// implement conversions form parry2d shape::Ball to bevy::math::primitives::Sphere
#[cfg(feature = "dim2")]
impl Into<shape::Ball> for primitives::Sphere {
    fn into(self) -> shape::Ball {
        shape::Ball::new(self.radius as Real)
    }
}

// implement conversions form parry3d shape::Ball to bevy::math::primitives::Sphere
#[cfg(feature = "dim3")]
impl Into<shape::Ball> for primitives::Sphere {
    fn into(self) -> shape::Ball {
        shape::Ball::new(self.radius as Real)
    }
}
