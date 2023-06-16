use super::hitable::*;
use super::hitable::Hitable;
use super::hitable::HitRecord;
use super::ray::*;
use super::vec::Vec3;

pub struct HitableList{
    pub spheres: Vec<Box<dyn Hitable>>
}

impl HitableList{
    pub fn new(list_size: usize) -> Self{
        return HitableList{spheres: Vec::with_capacity(list_size)};
    }

    pub fn add(&mut self, sphere: Box<dyn Hitable>){
        return self.spheres.push(sphere);
    }
    
}

impl Hitable for HitableList{
    fn hit(&self, r: Ray, t_min: f32,
           t_max: f32, rec: &mut HitRecord) -> bool{
        let mut hit_anything: bool = false;
        let mut closest_so_far = rec.t;
        let mut temp_record: HitRecord = HitRecord::new(0.0,Vec3::new(0.0,0.0,0.0),
                                             Vec3::new(0.0,0.0,0.0));
        
        
        
        for i in &self.spheres{
            
            let temp_result = i.hit(r, t_min, t_max, &mut temp_record.to_owned());
            // println!("dbg temp_record, {:?}", &temp_record.t);
            if temp_result == true{
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record;
                return hit_anything;
            };
        }
        
    return hit_anything;
    }
}