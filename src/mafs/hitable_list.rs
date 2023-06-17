use super::hitable::Hitable;
use super::hitable::HitRecord;
use super::ray::Ray;
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
           t_max: f32) -> Option<HitRecord>{
            let mut hit_anything = None;
            let mut closest_so_far = t_max;
            for i in &self.spheres{
                let temp_result = i.hit(r, t_min, closest_so_far);
                match temp_result{
                    Some(rec) =>{
                        closest_so_far = rec.t;
                        hit_anything = Some(rec);
                    }
                    None => {}
                }
            }
            return hit_anything;
    }
}