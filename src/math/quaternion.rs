use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Rotation: Sized {
    // begin state, no rotation
    fn identity() -> Self;

    // base operations
    fn conjugate(&self) -> Self;
    fn magnitude(&self) -> f32;
    fn normalize(&self) -> Self;

    // multiplication
    fn combine(&self, other: Self) -> Self;

    //
    fn rotate(&self, vector: Vec3) -> Vec3;

    // constructor
    fn from_axis_angle(axis: Vec3, angle: f32) -> Self;
    fn to_euler(&self) -> Vec3;
}
