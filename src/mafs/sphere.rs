use super::vec::Vec3;
use super::hitable::*;
use super::ray::*;
struct Sphere{
    pub center: Vec3,
    pub radius: f32
}

impl Sphere{
    pub fn new(center: Vec3, radius: f32) -> Sphere{
        return Sphere{center, radius};
    }
}

impl Hitable for Sphere{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
        let sqrtd: f32 = f32::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if (discriminant < 0.0){
            return false;
        }
        if(root < t_min && t_max < root){
            root = (-half_b + sqrtd)/a;
            if(root < t_min && t_max < root){
                return false;
            };
        };
        rec.t = root;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
    }
}