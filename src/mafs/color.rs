use super::vec::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Pixel_color{
   pub r: u32,
   pub g: u32,
   pub b: u32 
}
#[derive(Copy, Clone, Debug)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32
}
impl Pixel_color{
    pub fn new(r: u32, g: u32, b: u32) -> Pixel_color{
        return Pixel_color{r : r, g: g, b: b};
    }
}

impl Color{
    pub fn new(r: f32, g: f32, b: f32) -> Color{
        return Color{r: r, g: g, b: b};
    }

    pub fn add_color(&self, color: Color) -> Color{
        let ar = self.r + color.r;
        let ag = self.g + color.g;
        let ab = self.b + color.b;
        return Color{
            r: ar,
            g: ag,
            b: ab
        };
    }

    pub fn write_color(&self) -> Pixel_color{
        let pixel_r = (255.99 * self.r) as u32;
        let pixel_g = (255.99 * self.g) as u32;
        let pixel_b = (255.99 * self.b) as u32;
        return Pixel_color { r: pixel_r, g: pixel_g, b: pixel_b };
    }
}
