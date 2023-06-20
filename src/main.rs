pub mod mafs;
use std::char::MAX;

use mafs::hitable::HitRecord;
use mafs::{vec::Vec3, ray::Ray, 
            color::Color, 
            camera::Camera};
use mafs::{hitable_list::*, mafsconsts::*};
use mafs::sphere::Sphere;
use mafs::hitable::Hitable;

use crate::mafs::color::Pixel_color;
use crate::mafs::materials::Lambertian;
use crate::mafs::ray;
fn main() {
   render(); 
}


fn ray_color(r: Ray, world: &HitableList, depth: f64, max_depth: f64) -> Vec3{
    match world.hit(r, 0.001, INFINITY){
        
        Some(rec) =>{
            let mut scattered: Ray = Ray::new(Vec3::new(0.0,0.0,0.0), 
                                            Vec3::new(0.0,0.0,0.0));
            let mut attenuation: Vec3 = Vec3::new(0.0,0.0,0.0);
            if depth < max_depth && rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered){
                return attenuation * ray_color(scattered, world, depth+1.0, max_depth);
            } else{
                Vec3::new(0.0,0.0,0.0)
            }
            // if (depth <= 0.0){
            //     return Color{
            //         r: 0.0,
            //         g: 0.0,
            //         b: 0.0
            //     }
            // }
            // let target: Vec3 = rec.p + rec.normal + Vec3::random_unit_vector();
            
            // let final_color: Color =  ray_color(Ray::new(rec.p, target - rec.p), world, depth-1.0);
            // // println!("{:?}",&final_color);
            // return Color{
            //     r: 0.5 * final_color.r,
            //     g: 0.5 * final_color.g,
            //     b: 0.5 * final_color.b,
            // }
        }
        // no sphere = sky
        None => {
            let unit_direction = Vec3::make_unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0-t) * Vec3::new(1.0,1.0,1.0) + t*Vec3::new(0.5, 0.7, 1.0);
        }
    } 
} 



fn render(){
    const ASPECT_RATIO: f64 = 2.35;
    const HEIGHT: u32 = 50; 
    const WIDTH: u32 = (HEIGHT as f64 * (ASPECT_RATIO) ) as u32;
    const SAMPLES: f64 = 50.0;
    const MAX_DEPTH: f64 = 10.0;
    //P3 framebuffer
    println!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    let mut world: HitableList = HitableList::new(4);

    world.add(Box::new(Sphere::new(Vec3::new(4.2, 1.83, -3.0), 2.8, Box::new(Lambertian::new(Vec3::new(0.1,0.2,0.1))))));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.53, -3.0), 1.8, Box::new(Lambertian::new(Vec3::new(0.3,0.8,0.88))))));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, -3.0), 1.2, Box::new(Lambertian::new(Vec3::new(0.73,0.87,0.5))))));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.1, Box::new(Lambertian::new(Vec3::new(0.9,0.3,0.5))))));
    world.add(Box::new(Sphere::new(Vec3::new(0.4, 0.0, -1.4), 0.34, Box::new(Lambertian::new(Vec3::new(0.1,0.3,0.5))))));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.95,0.93,0.93))))));

    let cam: Camera = Camera::new(2.0,
                            ASPECT_RATIO*2.0,
                            Vec3::new(0.0,0.0,0.0),
                            1.0);
    for j in(0..HEIGHT).rev(){
        for i in 0..WIDTH{
            let mut wcol: Vec3 = Vec3::new(0.0,0.0,0.0);
            let mut final_color: Pixel_color = Pixel_color { r: 0, g: 0, b: 0 }; 
            for _samples in 0..SAMPLES as u32{
                let u: f64 = (i as f64 + randomf64())/(WIDTH-1) as f64;
                let v: f64 = (j as f64 + randomf64())/(HEIGHT-1) as f64;
                let r: Ray = cam.get_ray(u, v);
                wcol += ray_color(r, &world, 0.0, MAX_DEPTH); 
                final_color = wcol.write_color(wcol,SAMPLES);
            }
            
            println!("{:?} {:?} {:?}", final_color.r , final_color.g ,final_color.b);
        }
    }
}