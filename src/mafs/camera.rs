use super::{vec::Vec3, ray::Ray, mafsconsts};

pub struct Camera{
    pub view_h: f64,
    pub view_w: f64,
    pub origin: Vec3,
    pub focal_length: f64,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_c: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3
}

impl Camera{
    pub fn new(look_from: Vec3,
              look_at: Vec3,
              vup: Vec3, 
              vfov: f64,
              aspect: f64 
               ) -> Camera{
               let theta = vfov*mafsconsts::PI / 180.0;
               let horizontal: Vec3 = Vec3::new(view_w,0.0,0.0);
               let vertical: Vec3 = Vec3::new(0.0, view_h, 0.0);
               let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
               
               
               return Camera{
                    view_h: view_h,
                    view_w: view_w,
                    origin: origin,
                    focal_length: focal_length,
                    horizontal: horizontal,
                    vertical: vertical,
                    lower_left_c: lower_left_corner,
                };
               } 
    pub fn get_ray(&self,u: f64, v: f64) -> Ray{
        return Ray{
            origin: self.origin,
            direction: self.lower_left_c + u*self.horizontal + v*self.vertical - self.origin
        };
    }
}