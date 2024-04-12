use crate::{
    math::{Point, Real, Vector},
    shape,
};
#[cfg(feature = "bevy")]
use bevy::math::primitives;
use bevy::math::{Vec2, Vec3};

// CUBOID
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

// SPHERE
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

// CYLINDER
// implement conversions for bevy::math::primitives::Cylinder and parry2d shape::Ball
#[cfg(feature = "dim2")]
impl Into<shape::Ball> for primitives::Cylinder {
    fn into(self) -> shape::Ball {
        shape::Ball::new(self.radius as Real)
    }
}

// implement conversions for bevy::math::primitives::Cylinder and parry3d shape::Cylinder
#[cfg(feature = "dim3")]
impl Into<shape::Cylinder> for primitives::Cylinder {
    fn into(self) -> shape::Cylinder {
        shape::Cylinder::new(self.half_height as Real, self.radius as Real)
    }
}

// Triangle
// implement conversions for bevy::math::primitives::Triangle and parry2d shape::Triangle
#[cfg(feature = "dim2")]
impl Into<shape::Triangle> for primitives::Polygon<3> {
    fn into(self) -> shape::Triangle {
        shape::Triangle::new(
            Point::from([self.vertices[0].x as Real, self.vertices[0].y as Real]),
            Point::from([self.vertices[1].x as Real, self.vertices[1].y as Real]),
            Point::from([self.vertices[2].x as Real, self.vertices[2].y as Real]),
        )
    }
}

// // implement conversions for bevy::math::primitives::Triangle and parry3d shape::Triangle
// #[cfg(feature = "dim3")]
// impl Into<shape::Triangle> for primitives::Polygon<3> {
//     fn into(self) -> shape::Triangle {
//         shape::Triangle::new(
//             Point::from([
//                 self.vertices[0].x as Real,
//                 self.vertices[0].y as Real,
//                 self.vertices[0].z as Real,
//             ]),
//             Point::from([
//                 self.vertices[1].x as Real,
//                 self.vertices[1].y as Real,
//                 self.vertices[1].z as Real,
//             ]),
//             Point::from([
//                 self.vertices[2].x as Real,
//                 self.vertices[2].y as Real,
//                 self.vertices[2].z as Real,
//             ]),
//         )
//     }
// }
