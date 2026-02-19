use core::ops::{Add, Div, Mul, Sub};
use libm::{acosf, sqrtf};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Vector: Sized + Copy + Add + Sub + Mul<f32> + Div<f32> {
    // static constructor method
    fn from_array(array: [f32; 3]) -> Self;
    fn zero() -> Self;
    fn one() -> Self;

    // base operations
    fn dot(&self, other: Self) -> f32;
    fn cross(&self, other: Self) -> Self;
    fn magnitude(&self) -> f32;
    fn magnitude_squared(&self) -> f32;
    fn normalize(&self) -> Self;

    // distance and orientation
    fn distance(&self, other: Self) -> f32;
    fn distance_squared(&self, other: Self) -> f32;
    fn limit(&self, max: f32) -> Self;
    fn lerp(&self, other: Self, t: f32) -> Self;

    // pyhsics
    fn reflect(&self, normal: Self) -> Self;
    fn refract(&self, normal: Self, etai_over_etat: f32) -> Self;

    // angle calculation
    fn angle(&self, other: Self) -> f32;
    fn angle_signed(&self, other: Self, normal: Self) -> f32;

    // data conversion
    fn to_array(&self) -> [f32; 3];
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Vector for Vec3 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    fn from_array(array: [f32; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }

    fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn magnitude(&self) -> f32 {
        sqrtf(self.dot(*self))
    }

    fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Self::zero();
        }
        *self * (1.0 / mag)
    }

    fn distance(&self, other: Self) -> f32 {
        (*self - other).magnitude()
    }

    fn distance_squared(&self, other: Self) -> f32 {
        (*self - other).magnitude_squared()
    }

    fn limit(&self, max: f32) -> Self {
        let mag_sq = self.magnitude_squared();
        if mag_sq > max * max {
            *self * (max / sqrtf(mag_sq))
        } else {
            *self
        }
    }

    fn lerp(&self, other: Self, t: f32) -> Self {
        Self {
            x: (1.0 - t) * self.x + t * other.x,
            y: (1.0 - t) * self.y + t * other.y,
            z: (1.0 - t) * self.z + t * other.z,
        }
    }

    fn reflect(&self, normal: Self) -> Self {
        *self - normal * (2.0 * self.dot(normal))
    }

    fn refract(&self, normal: Self, etai_over_etat: f32) -> Self {
        let cos_theta = (-self.dot(normal)).min(1.0).max(-1.0);
        let r_out_perp = (*self + normal * cos_theta) * etai_over_etat;
        let k = 1.0 - r_out_perp.magnitude_squared();
        if k < 0.0 {
            // total internal reflection — kırılma yok
            return Self::zero();
        }
        let r_out_parallel = normal * -sqrtf(k);
        r_out_perp + r_out_parallel
    }

    fn angle(&self, other: Self) -> f32 {
        let denom = self.magnitude() * other.magnitude();
        if denom == 0.0 {
            return 0.0;
        }
        let cos_angle = (self.dot(other) / denom).min(1.0).max(-1.0);
        acosf(cos_angle)
    }

    fn angle_signed(&self, other: Self, normal: Self) -> f32 {
        let angle = self.angle(other);
        if self.cross(other).dot(normal) < 0.0 {
            -angle
        } else {
            angle
        }
    }

    fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
