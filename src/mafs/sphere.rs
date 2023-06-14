use super::vec::Vec3;
use super::hitable::*;
use super::ray::*;
struct Sphere{
    center: Vec3,
    radius: f32
}

impl Sphere{
    pub fn new(center: Vec3, radius: f32) -> Sphere{
        return Sphere{center, radius};
    }
}

impl Hitable for Sphere{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let oc:Vec3 = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius*self.radius;
    }
}