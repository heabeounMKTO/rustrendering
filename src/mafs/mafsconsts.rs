pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.141592653589;
use rand::prelude::*;
pub fn degrees_to_radians(degrees: f32) -> f32{
    return degrees * PI / 180.0;
}

pub fn randomf32() -> f32{
    let mut rng = rand::thread_rng();
    return rng.gen();
}

pub fn randomf32_range(min: f32, max: f32) -> f32{
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);  
}

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {return min;};
    if input > max {return max;}
    return input;
}