 use super::vec::Vec3;
 use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
 pub struct HitRecord{
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
   //  pub front_face: bool
 }

 impl HitRecord{
    pub fn new(t:f64 ,p: Vec3, normal: Vec3) -> Self{
        return HitRecord { t: 0.0, 
                           p: Vec3::new(0.0,0.0,0.0),
                           normal: Vec3::new(0.0,0.0,0.0)
                           };
    }

 }
 pub trait Hitable{
    fn hit(&self, t: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
 }