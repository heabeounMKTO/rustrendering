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

pub struct Metal{
    albedo: Vec3,
    roughness: f64
}

impl Lambertian{
    pub fn new(albedo: Vec3) -> Self{
        return Lambertian{albedo};
    }
}

impl Metal{
    pub fn new(albedo: Vec3, roughness: f64) -> Self{
        return Metal { albedo: albedo, roughness: f64::min(roughness, 1.0)};
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

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool{
        let reflected: Vec3 = refelct(Vec3::make_unit_vector(r_in.direction())
                                      , rec.normal);
        *scattered = Ray::new(rec.p, reflected+self.roughness*Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
    fn clone(&self) -> Box<dyn Material>{
        return Box::new(Metal::new(self.albedo, self.roughness));
    }
}

pub fn refelct(v: Vec3, n: Vec3) -> Vec3{
    return v - 2.0*Vec3::dot(&v, &n)*n;
}