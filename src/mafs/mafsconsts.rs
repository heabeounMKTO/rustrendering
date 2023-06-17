pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.141592653589;

pub fn degrees_to_radians(degrees: f32) -> f32{
    return degrees * PI / 180.0;
}