use std::fs::File;
use std::io::Write;
use std::ops::{Neg, AddAssign, MulAssign, DivAssign, SubAssign, Add, Sub};

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

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Vec3 {
        Vec3{ x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Vec3{ x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
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

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    fn at(self, t: f64) -> Vec3 {
        return self.origin + self.dir.multiply_by(t);
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

fn unit_vector(v: Vec3) -> Vec3 {
    divide_by(v, v.length())
}

fn ray_colour(r: Ray) -> Vec3 {
    let unit_direction = unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    let one_minus_t = 1.0 - t;
    let out = Vec3{x: 1.0, y:  1.0, z: 1.0}.multiply_by(one_minus_t) + (Vec3{x: 0.5, y: 0.7, z: 1.0}.multiply_by(t));
    println!("out: {:?}",out);
    return out;
}

fn write_colour(buffer: &mut File, pixel_colour: Vec3) {

    let ir = (255.999 * pixel_colour.x) as i32;
    let ig = (255.999 * pixel_colour.y) as i32;
    let ib = (255.999 * pixel_colour.z) as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    buffer.write(&pixel.as_bytes());
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{ x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{ x: 0.0,  y: viewport_height, z: 0.0};
    let lower_left_corner = origin - divide_by(horizontal, 2.0) - divide_by(vertical, 2.0) - Vec3{ x: 0.0, y: 0.0, z: focal_length};
    println!("lower_left_corner: {:?}", lower_left_corner);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut buffer = File::create("output.ppm")?;
    buffer.write(&header.as_bytes());

    for j in (0..(image_height-1)).rev() {
        println!("\r scan lines remaining: {}", j);
        for i in 0..image_width {
            // let r = i as f64 / (image_width-1) as f64;
            // let g = j as f64 / (image_height-1) as f64;
            // let b = 0.25;

            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let temp_h = horizontal.multiply_by(u);
            let temp_v = vertical.multiply_by(v);
            let r = Ray{ origin: origin, dir: (lower_left_corner + temp_h + temp_v) - origin};

            let pixel_colour = ray_colour(r);
            write_colour(&mut buffer, pixel_colour);
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
    fn test_length() {
        let v1 = Vec3{ x: 2.0, y: 2.0, z: 2.0 };
        assert_eq!(3.4641016151377544, v1.length());
        let v2 = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!(1.7320508075688772, v2.length())
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

    #[test]
    fn test_at() {
        let origin = Vec3{ x: 0.0,  y: 0.0, z: 0.0 };
        let dir = Vec3{ x: 1.0, y: 0.0, z: 0.0 };

        let r = Ray{ origin: origin, dir: dir };

        let expected = Vec3{ x: 1.0, y: 0.0, z: 0.0 };
        assert_eq!(expected, r.at(1.0));
    }
}