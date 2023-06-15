use super::vec::Vec3;

struct Camera{
    view_h: f32,
    view_w: f32,
    origin: Vec3,
    focal_length: f32,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_c: Vec3
}

impl Camera{
    pub fn new(view_h: f32, view_w: f32,
               origin: Vec3, focal_length: f32) -> Camera{
                return Camera;
               } 
}