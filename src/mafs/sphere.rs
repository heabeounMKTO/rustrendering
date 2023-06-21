use super::materials::Material;
use super::vec::Vec3;
use super::hitable::*;
use super::ray::*;
pub struct Sphere{
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>
}

impl Sphere{
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere{
        return Sphere{center, radius, material};
    }
}

impl Hitable for Sphere{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = (half_b*half_b - a*c).abs(); // added abs for debugging
        if discriminant > 0.0{
           let temp = (-half_b - f64::sqrt(half_b*half_b - a*c))/a;
           if temp < t_max && temp > t_min{
            let t = temp;
            let p = r.point_at_parameter(t);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord{t, p, normal, material: self.material.clone()});
           };
           let temp = (-half_b + f64::sqrt(half_b*half_b-a*c))/a;
           if temp < t_max && temp > t_min{
            let t = temp;
            let p = r.point_at_parameter(t);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord{t, p, normal, material: self.material.clone()});
           };

        }
       None 

    }
}