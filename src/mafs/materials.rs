use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec::Vec3;
use super::color::Color;

pub trait Material{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, 
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    fn clone(&self) -> Box<dyn Material>;
}

pub struct Lambertian{
    albedo: Vec3
}

impl Lambertian{
    pub fn new(albedo: Vec3) -> Self{
        return Lambertian{albedo};
    }
}

impl Material for Lambertian{
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool{
                let target: Vec3 = rec.p + rec.normal + Vec3::random_in_unit_sphere();
                *scattered = Ray::new(rec.p, target-rec.p);
                *attenuation = self.albedo;
                return true;
               }
    fn clone(&self) -> Box<dyn Material>{
        return Box::new(Lambertian::new(self.albedo));
    }
}