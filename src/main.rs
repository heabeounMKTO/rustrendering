pub mod mafs;

use mafs::{vec::Vec3, ray::Ray, color::Pixel_color, color::Color, camera::Camera};


fn main() {
   render(); 
}


fn ray_color(ray: Ray) -> Color{
    let t :f32 = hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.4, ray);
    if (t > 0.0){
        let N: Vec3 = Vec3::make_unit_vector(
                        ray.point_at_parameter(t) - Vec3::new(0.0,0.0,-1.0));
        return Color{
                r: 0.5*(N.x()+1.0), 
                g: 0.5*(N.y()+1.0),
                b: 0.5*(N.z()+1.0),
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