use super::{vec::Vec3, ray::Ray};

pub struct Camera{
    pub view_h: f32,
    pub view_w: f32,
    pub origin: Vec3,
    pub focal_length: f32,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_c: Vec3
}

impl Camera{
    pub fn new(view_h: f32, 
               view_w: f32,
               origin: Vec3, 
               focal_length: f32) -> Camera{
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
    pub fn get_ray(&self,u: f32, v: f32) -> Ray{
        return Ray{
            origin: self.origin,
            direction: self.lower_left_c + u*self.horizontal + v*self.vertical - self.origin
        };
    }
}