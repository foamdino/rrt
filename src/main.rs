use std::fs::File;
use std::io::Write;
use std::ops::{Neg, AddAssign, MulAssign, DivAssign, SubAssign, Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

type Colour = Vec3;
type Point = Vec3;

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

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}

// impl Mul<i32> for Vec3 {
//     type Output = Vec3;
//
//     fn mul(self, other: i32) -> Vec3 {
//         Vec3 {
//             x: self.x * other as f64,
//             y: self.y * other as f64,
//             z: self.z * other as f64
//         }
//     }
// }
//
// impl Mul<Vec3> for i32 {
//     type Output = Vec3;
//
//     fn mul(self, rhs: Vec3) -> Vec3 {
//         Vec3{
//             x: rhs.x * self as f64,
//             y: rhs.y * self as f64,
//             z: rhs.z * self as f64
//         }
//     }
// }

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: (self.x / other.x),
            y: (self.y / other.y),
            z: (self.z / other.z)
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    fn at(self, t: f64) -> Vec3 {
        return self.origin + self.dir.multiply_by(t);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64
}

pub trait SceneObject {
    fn hittable(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
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
    multiply_by(v, 1.0/t)
}

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

fn hit_sphere(p: Point, radius: f64, r: Ray) -> f64 {
    let oc = r.origin - p;
    let a = r.dir.length_squared();
    let half_b = dot(oc, r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        -half_b - discriminant.sqrt() / a
    }
}

fn ray_colour(r: Ray) -> Vec3 {
    let t = hit_sphere(Point{x: 0.0, y: 0.0, z: -1.0}, 0.5, r);
    // if t > 0 then we intersected with a point on the sphere
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3{x:0.0, y:0.0, z:-1.0});
        0.5 * Colour{x: n.x+1.0, y: n.y+1.0, z: n.z+1.0}
        //Colour{x:0.0, y:0.0, z:0.0}
    } else { // we generate a background colour
        let unit_direction = unit_vector(r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        // let out = Colour{x: 1.0, y:  1.0, z: 1.0}.multiply_by(one_minus_t) + (Colour{x: 0.5, y: 0.7, z: 1.0}.multiply_by(t));
        // println!("r: {:?}",r);
        // println!("unit dir: {:?}",unit_direction);
        // println!("t: {:?}",t);
        // println!("t: {:?}",t.ceil());
        // println!("color: {:?}",(1.0 - t) * Colour{x: 1.0, y: 1.0, z: 1.0} + t * Colour{x: 0.5, y: 0.7, z: 1.0});
        (1.0 - t) * Colour{x: 1.0, y: 1.0, z: 1.0} + t * Colour{x: 0.5, y: 0.7, z: 1.0}
    }
}

fn write_colour(buffer: &mut File, pixel_colour: Vec3) {

    let ir = (255.999 * pixel_colour.x).ceil() as i32;
    let ig = (255.999 * pixel_colour.y).ceil() as i32;
    let ib = (255.999 * pixel_colour.z).ceil() as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    buffer.write(&pixel.as_bytes()).expect("unable to write to buffer");
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).ceil() as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{ x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{ x: 0.0,  y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3{ x: 0.0, y: 0.0, z: focal_length};
    // println!("lower_left_corner: {:?}", lower_left_corner);
    // println!("hor / 2: {:?}", horizontal / 2.0);
    // println!("ver / 2: {:?}", vertical / 2.0);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut buffer = File::create("output.ppm")?;
    buffer.write(&header.as_bytes()).expect("unable to write to buffer");

    for j in (0..image_height).rev() {
        println!("\r scan lines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray{ origin: origin, dir: lower_left_corner + u*horizontal + v*vertical - origin};

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