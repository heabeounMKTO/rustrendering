pub mod mafs;
use mafs::hitable::HitRecord;
use mafs::{vec::Vec3, ray::Ray, 
            color::Color, 
            camera::Camera};
use mafs::{hitable_list::*, mafsconsts::*};
use mafs::sphere::Sphere;
use mafs::hitable::Hitable;

use crate::mafs::color::Pixel_color;
use crate::mafs::ray;
fn main() {
   render(); 
}


fn ray_color(r: Ray, world: &HitableList) -> Color{
    match world.hit(r, 0.0, INFINITY){
        // if there is sphere , return normals
        Some(rec) =>{
            return Color{
                r: 0.5 * (rec.normal.x() + 1.0),
                g: 0.5 * (rec.normal.y() + 1.0),
                b: 0.5 * (rec.normal.z() + 1.0),
            }
        }
        // no sphere = sky
        None => {
            let unit_direction = Vec3::make_unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Color{
                r: (1.0-t) * 1.0 + t*0.5,
                g: (1.0-t) * 1.0 + t*0.7,
                b: (1.0-t) * 1.0 + t*1.0,
            }
        }
    } 
} 



fn render(){
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const HEIGHT: u32 = 400; 
    const WIDTH: u32 = (HEIGHT as f32 * (ASPECT_RATIO) ) as u32;
    const SAMPLES: f32 = 2.0;
    //P3 framebuffer
    println!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    let mut world: HitableList = HitableList::new(4);
    // world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(0.4, 0.0, -1.4), 0.34)));
    world.add(Box::new(Sphere::new(Vec3::new(-0.67, 1.0, -3.4), 1.4)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam: Camera = Camera::new(2.0,
                            ASPECT_RATIO*2.0,
                            Vec3::new(0.0,0.0,0.0),
                            1.0);
    for j in(0..HEIGHT).rev(){
        for i in 0..WIDTH{
            let mut wcol: Color = Color::new(0.0,0.0,0.0);
            let mut final_color: Pixel_color = Pixel_color { r: 0, g: 0, b: 0 }; 
            for _samples in 0..SAMPLES as u32{
                let u: f32 = (i as f32 + randomf32())/(WIDTH-1) as f32;
                let v: f32 = (j as f32 + randomf32())/(HEIGHT-1) as f32;
                let r: Ray = cam.get_ray(u, v);
                wcol = Color::add_color(wcol,ray_color(r, &world));
                final_color = wcol.write_color(wcol,SAMPLES);
            }
            
            println!("{:?} {:?} {:?}", final_color.r , final_color.g ,final_color.b);
        }
    }
}