use super::vec::Vec3;
use super::mafsconsts::clamp;
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

    pub fn add_color(color: Color, color1: Color) -> Color{
        let ar = color.r + color1.r;
        let ag = color.g + color1.g;
        let ab = color.b + color1.b;
        return Color{
            r: ar,
            g: ag,
            b: ab
        };
    }

    pub fn write_color(&self, pixel_color: Color ,  samples: f32) -> Pixel_color{
        let mut r: f32 = pixel_color.r;
        let mut g: f32 = pixel_color.g;
        let mut b: f32 = pixel_color.b;

        let scale: f32 = 1.0 / samples;
        r *= scale;
        g *= scale;
        b *= scale;
        let pixel_r =  (256.0 * clamp(r, 0.0, 0.999)) as u32;
        let pixel_g =  (256.0 * clamp(g, 0.0, 0.999)) as u32;
        let pixel_b =  (256.0 * clamp(b, 0.0, 0.999)) as u32;
        return Pixel_color { r: pixel_r, g: pixel_g, b: pixel_b };
    }
}
