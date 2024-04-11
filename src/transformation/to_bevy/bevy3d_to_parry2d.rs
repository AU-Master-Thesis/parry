use crate::{
    math::{Real, Vector},
    shape,
};
#[cfg(feature = "bevy")]
use bevy::math::primitives;
use bevy::math::{Vec2, Vec3};

// implement conversions for bevy::math::primitives::Cuboid and parry2d shape::Cuboid
#[cfg(feature = "dim2")]
impl Into<shape::Cuboid> for primitives::Cuboid {
    fn into(self) -> shape::Cuboid {
        let (x, y) = (self.half_size.x as Real, self.half_size.z as Real);
        shape::Cuboid::new(Vector::new(x, y))
    }
}

// implement conversions for bevy::math::primitives::Cuboid and parry3d shape::Cuboid
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

#[cfg(feature = "dim3")]
impl Into<primitives::Cuboid> for shape::Cuboid {
    fn into(self) -> primitives::Cuboid {
        let (x, y, z) = (
            self.half_extents.x as f32,
            self.half_extents.y as f32,
            self.half_extents.z as f32,
        );
        primitives::Cuboid {
            half_size: Vec3::new(x, y, z),
        }
    }
}

// implement conversions for bevy::math::primitives::Sphere and parry2d shape::Ball
#[cfg(feature = "dim2")]
impl Into<shape::Ball> for primitives::Sphere {
    fn into(self) -> shape::Ball {
        shape::Ball::new(self.radius as Real)
    }
}

// implement conversions for bevy::math::primitives::Sphere and parry3d shape::Ball
#[cfg(feature = "dim3")]
impl Into<shape::Ball> for primitives::Sphere {
    fn into(self) -> shape::Ball {
        shape::Ball::new(self.radius as Real)
    }
}
