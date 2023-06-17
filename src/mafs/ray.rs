use super::vec::Vec3;

#[derive(Copy,Clone,Debug)]
pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray{
    pub fn new(origin: Vec3, direction: Vec3) -> Ray{
        return Ray{origin, direction};
    }

    pub fn origin(&self) -> Vec3{
        return self.origin;
    }

    pub fn direction(&self) -> Vec3{
        return self.direction;
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3{
        return self.origin + t*self.direction;
    }
}