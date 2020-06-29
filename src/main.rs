use std::fs::File;
use std::io::Write;
use std::ops::{Neg, AddAssign, MulAssign, DivAssign, SubAssign, Add};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn multiply_by(&self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t
        }
    }

    pub fn divide_by(&self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{ x: -self.x, y: -self.y, z: -self.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        };
    }
}

// Utility Functions
fn add(u: Vec3, v: Vec3) -> Vec3 {
    Vec3{ x: u.x + v.x, y: u.y + v.y, z: u.z + v.z }
}

fn sub(u: Vec3, v: Vec3) -> Vec3 {
    Vec3{ x: u.x - v.x, y: u.y - v.y, z: u.z - v.z }
}

fn mul(u: Vec3, v: Vec3) -> Vec3 {
    Vec3{ x: u.x * v.x, y: u.y * v.y, z: u.z * v.z }
}

fn multiply_by(v: Vec3, t: f64) -> Vec3 {
    Vec3{ x: v.x * t, y: v.y * t, z: v.z * t }
}

fn divide_by(v: Vec3, t: f64) -> Vec3 {
    multiply_by(v, (1/t as i32) as f64)
}

// fn write_colour(buffer: &mut File, pixel: Vec3) {
//
//     buffer.write(&pixel.as_bytes());
// }

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut buffer = File::create("output.ppm")?;
    buffer.write(&header.as_bytes());

    for j in (0..(image_height-1)).rev() {
        println!("\r scan lines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_height-1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let pixel = format!("{} {} {}\n", ir, ig, ib);
            buffer.write(&pixel.as_bytes());
        }
    }

    println!("Fin!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vec3;

    #[test]
    fn test_length_squared() {
        let v1 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!(3.0, v1.length_squared());
        let v2 = Vec3{ x: 2.0, y: 2.0, z: 2.0 };
        assert_eq!(12.0, v2.length_squared());
    }

    #[test]
    fn test_negation() {
        let v1 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        let v2 = -v1;
        println!("{:?}",&v1);
        println!("{:?}",&v2);
        assert_eq!(-1.0, v2.x);
        assert_eq!(-1.0, v2.y);
        assert_eq!(-1.0, v2.z);
    }

    #[test]
    fn test_add_assign() {
        let v1 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        let mut v2 = Vec3{ x: 1.0, y: 2.0, z: 3.0 };
        v2 += v1;
        let expected = Vec3{ x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(expected, v2);
    }

    #[test]
    fn test_sub_assign() {
        let v1 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        let mut v2 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        v2 -= v1;
        let expected = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
        assert_eq!(expected, v2);
    }

    #[test]
    fn test_mul_assign() {
        let v1 = Vec3{ x: 2.0, y: 2.0, z: 2.0 };
        let mut v2 = Vec3{ x: 3.0, y: 3.0, z: 3.0 };
        v2 *= v1;
        let expected = Vec3{ x: 6.0, y: 6.0, z: 6.0 };
        assert_eq!(expected, v2);
    }

    #[test]
    fn test_div_assign() {
        let v1 = Vec3{ x: 2.0, y: 2.0, z: 2.0 };
        let mut v2 = Vec3{ x: 6.0, y: 6.0, z: 6.0 };
        v2 /= v1;
        let expected = Vec3{ x: 3.0, y: 3.0, z: 3.0 };
        assert_eq!(expected, v2);
    }

    #[test]
    fn test_multiply_by() {
        let v1 = Vec3{ x: 2.0, y: 2.0, z: 2.0 };
        let v2 = v1.multiply_by(3.0);
        let expected = Vec3{ x: 6.0, y: 6.0, z: 6.0};
        assert_eq!(expected, v2);
    }

    #[test]
    fn test_divide_by() {
        let v1 = Vec3{ x: 6.0, y: 6.0, z: 6.0 };
        let v2 = v1.divide_by(3.0);
        let expected = Vec3{ x: 2.0, y: 2.0, z: 2.0};
        assert_eq!(expected, v2);
    }

    #[test]
    fn test_add_fn() {
        let v1 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        let v2 = Vec3{ x: 1.0, y: 2.0, z: 3.0 };
        let v3 = add(v2, v1);
        let expected = Vec3{ x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(expected, v3);
    }

    #[test]
    fn test_sub_fn() {
        let v1 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        let v2 = Vec3{ x: 1.0, y: 2.0, z: 3.0 };
        let v3 = sub(v2, v1);
        let expected = Vec3{ x: 0.0, y: 1.0, z: 2.0 };
        assert_eq!(expected, v3);
    }

    #[test]
    fn test_mul_fn() {
        let v1 = Vec3{ x: 2.0, y: 2.0, z: 2.0 };
        let v2 = Vec3{ x: 1.0, y: 2.0, z: 3.0 };
        let v3 = mul(v2, v1);
        let expected = Vec3{ x: 2.0, y: 4.0, z: 6.0 };
        assert_eq!(expected, v3);
    }
}