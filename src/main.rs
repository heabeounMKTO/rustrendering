pub mod mafs;

use mafs::hitable::HitRecord;
use mafs::{vec::Vec3, ray::Ray, 
            color::Color, 
            camera::Camera};
use mafs::{hitable_list::*, mafsconsts::INFINITY};
use mafs::sphere::Sphere;
use mafs::hitable::Hitable;
fn main() {
   render(); 
}


fn ray_color(r: Ray, world: &HitableList) -> Color{
    match world.hit(r, 0.0, INFINITY){
        Some(rec) =>{
            return Color{
                r: 0.5 * (rec.normal.x() + 1.0),
                g: 0.5 * (rec.normal.y() + 1.0),
                b: 0.5 * (rec.normal.z() + 1.0),
            }
        }
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


fn hit_sphere(center: Vec3, radius: f32, r:Ray) -> f32{
    let oc: Vec3 = r.origin() - center;
    let a = r.direction().squared_length();
    let half_b = Vec3::dot(&oc, &r.direction());
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b*half_b - 4.0*a*c;
    if(discriminant < 0.0){
        return -1.0;
    }else{
        return(-half_b - f32::sqrt(discriminant)) / (2.0*a);
    }
}
fn render(){
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const HEIGHT: u32 = 400; 
    const WIDTH: u32 = (HEIGHT as f32 * (ASPECT_RATIO) ) as u32;
    //P3 framebuffer
    println!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    let mut world: HitableList = HitableList::new(2);
    // world.add(Box::new
    //          (Sphere::new(Vec3::new(1.0, 0.0, 0.5), 0.5)));
    
    
    // let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    // let horizontal = Vec3::new(4.0, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, 2.0, 0.0);
    // let origin = Vec3::new(0.0, 0.0, 0.0);
    
    
    world.add(Box::new(Sphere::new(Vec3::new(0.2, 0.0, -0.2), 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    
    for j in(0..HEIGHT).rev(){
        for i in 0..WIDTH{
            let u: f32 = i as f32/(WIDTH-1) as f32;
            let v: f32 = j as f32/(HEIGHT-1) as f32;
            let cam: Camera = Camera::new(2.0,
                                          ASPECT_RATIO*2.0,
                                          Vec3::new(0.0,0.0,0.0),
                                          2.0);
            let penisRay: Ray = Ray::new(cam.origin,
                                         cam.lower_left_c + u*cam.horizontal + v*cam.vertical);
            let peniscolor: Color = ray_color(penisRay, &world);
            
            let write_col = peniscolor.write_color();
            println!("{:?} {:?} {:?}", write_col.r , write_col.g ,write_col.b);
        }
    }
}