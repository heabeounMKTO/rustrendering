use super::mafsconsts::{randomf64, randomf64_range};
use super::color_transforms::*;
use super::color::Pixel_color;
use super::mafsconsts::*;
#[derive(Copy,Clone,Debug)]
pub struct Vec3{
    pub e: [f64; 3],
}

impl Vec3{
    pub fn new(x: f64, y:f64, z: f64) -> Vec3{
         return Vec3{e:[x, y, z]};
    }
    pub fn x(&self) -> f64{ self.e[0] }
    pub fn y(&self) -> f64{ self.e[1] }
    pub fn z(&self) -> f64{ self.e[2] }
    pub fn r(&self) -> f64{ self.e[0] }
    pub fn g(&self) -> f64{ self.e[1] }
    pub fn b(&self) -> f64{ self.e[2] }
    
    pub fn length(&self) -> f64{
        f64::sqrt(
            self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2]
        )
    }
    pub fn squared_length(&self)->f64{
        self.e[0] * self.e[0] +
        self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }
    pub fn random(&self) -> Vec3{
        return Vec3{e:[randomf64(), randomf64(), randomf64()]};
    }

    pub fn random_range(min: f64, max: f64) -> Vec3{
        return Vec3{e:[randomf64_range(min, max), 
                      randomf64_range(min, max), 
                      randomf64_range(min,max)]};
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3{
        Vec3::new(
            v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
            -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
            v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0]
        )
    }

    pub fn random_in_unit_sphere() -> Vec3{
        let p: Vec3; 
       loop{
            let p: Vec3 = Vec3::random_range(-1.0,1.0);
            if p.squared_length() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3{
        let fuck: Vec3 = Vec3::random_in_unit_sphere();
        return Vec3::make_unit_vector(fuck);
    }
    pub fn write_color(&self, pixel_color: Vec3 ,  samples: f64) -> Pixel_color{
        let mut r: f64 = pixel_color.x();
        let mut g: f64 = pixel_color.y();
        let mut b: f64 = pixel_color.z();


        let scale: f64 = 1.0 / samples;
        //apply aces on linear color
        // got transform func from shadertoy somewhere lol
        r = apply_aces(scale * r);
        g = apply_aces(scale * g);
        b = apply_aces(scale * b);
    

        let pixel_r =  (256.0 * clamp(r, 0.0, 1.0)) as u32;
        let pixel_g =  (256.0 * clamp(g, 0.0, 1.0)) as u32;
        let pixel_b =  (256.0 * clamp(b, 0.0, 1.0)) as u32;
        return Pixel_color { r: pixel_r, g: pixel_g, b: pixel_b };
    }
    pub fn make_unit_vector(v : Vec3)->Vec3{
        v / v.length()
    }

    pub fn dot(v1 : &Vec3, v2 : &Vec3)->f64{
        v1.e[0] * v2.e[0] +
        v1.e[1] * v2.e[1] +
        v1.e[2] * v2.e[2]
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl std::ops::Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Vec3{
        Vec3::new(
            -self.e[0],
            -self.e[1],
            -self.e[2]
        )
    }
}

impl std::ops::Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, rhs:Vec3) -> Vec3{
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2]
        )
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2]
        )
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2]
        )
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs
        )
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self * rhs.e[0],
            self * rhs.e[1],
            self * rhs.e[2]
        )
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2]
        )
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs
        )
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.e[0] += other;
        self.e[1] += other;
        self.e[2] += other;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl std::ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.e[0] -= other;
        self.e[1] -= other;
        self.e[2] -= other;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}