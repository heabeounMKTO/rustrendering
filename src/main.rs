pub mod mafs;

use mafs::{vec::Vec3, ray::Ray, color::Pixel_color, color::Color, camera::Camera};


fn main() {
   render(); 
}


fn ray_color(ray: Ray) -> Color{
    if(hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, ray)) {
        return Color{
            r: 0.5,
            g:0.5,
            b:0.5
        }
    }
    let unit_direction: Vec3 = Vec3::make_unit_vector(ray.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    let bg: Color = Color::new(1.0,1.0,1.0);
    let fg: Color = Color::new(0.3, 0.7, 1.0);
    return Color 
            { r: (1.0-t)*bg.r + t*fg.r, 
              g: (1.0-t)*bg.g + t*fg.g, 
              b: (1.0-t)*bg.b + t*fg.b 
           } ; 
        
}

fn hit_sphere(center: Vec3, radius: f32, r:Ray) -> bool{
    let oc: Vec3 = r.origin() - center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0*Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    return(discriminant > 0.0);
}
fn render(){
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const HEIGHT: u32 = 400; 
    const WIDTH: u32 = (HEIGHT as f32 * (ASPECT_RATIO) ) as u32;
    //P3 framebuffer
    println!("P3\n{} {}\n255\n", WIDTH, HEIGHT);

    for j in(0..HEIGHT).rev(){
        for i in 0..WIDTH{
            let u: f32 = i as f32/(WIDTH-1) as f32;
            let v: f32 = j as f32/(HEIGHT-1) as f32;
            let cam: Camera = Camera::new(2.0, ASPECT_RATIO*2.0, Vec3::new(0.0,0.0,0.0), 1.0);
            let penisRay: Ray = Ray::new(cam.origin, cam.lower_left_c + u*cam.horizontal 
                                        + v*cam.vertical - cam.origin);
            let peniscolor: Color = ray_color(penisRay);
            let write_col = peniscolor.write_color();
            println!("{:?} {:?} {:?}", write_col.r , write_col.g ,write_col.b);
        }
    }
}