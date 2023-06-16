use super::vec::Vec3;
use super::hitable::*;
use super::ray::*;
pub struct Sphere{
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
        let discriminant = (half_b*half_b - a*c).abs(); // added abs for debugging
        let sqrtd: f32 = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        
        // println!("dbg sphere {:?}", &root);
        if (root < t_min || t_max < root){
            root = (-half_b + sqrtd)/a;
            if(root < t_min || t_max < root){
                return false;
            };
        };
        rec.t = root;
        rec.p = r.point_at_parameter(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center)/self.radius;
        
        rec.set_face_normal(r, outward_normal);
        return true;
    }
}