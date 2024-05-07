use super::{vec::Vec3, ray::Ray, mafsconsts};

pub struct Camera{
    origin: Vec3,
    lower_left_c: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
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
               ) -> Self{
               let theta = vfov*mafsconsts::PI / 180.0;
               let half_height = f64::tan(theta/2.0);
               let half_width = aspect*half_height;
               let origin = look_from;
               let w = Vec3::make_unit_vector(look_from-look_at);
               let u = Vec3::make_unit_vector(Vec3::cross(vup, w));
               let v = Vec3::cross(w,u);
               let mut lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
               let horizontal = 2.0*half_width*u;
               let vertical = 2.0*half_height*v;
               
               return Camera{
                    origin: origin,
                    lower_left_c: lower_left_corner,
                    horizontal: horizontal,
                    vertical: vertical,
                    u: u,
                    v: v,
                    w: w
                };
               } 
    pub fn get_ray(&self,u: f64, v: f64) -> Ray{
        return Ray{
            origin: self.origin,
            direction: self.lower_left_c + u*self.horizontal + v*self.vertical - self.origin
        };
    }
}