pub mod mafs;
use mafs::vec::Vec3;

fn main() {
   render(); 
}

fn render(){
    //P3 framebuffer
    let width: u32 = 1024;
    let height: u32 = 768;
    println!("P3\n{} {}\n255\n", width, height);

    for j in(0..height).rev(){
        for i in 0..width{
            let mut ayylmao: Vec3 = Vec3::new(j as f32 / width as f32
                                            , i as f32/ height as f32,
                                             0.2);
            let mut ir = (255.0 * ayylmao[0]) as u32;
            let mut ig = (255.0 * ayylmao[1]) as u32;
            let mut ib = (255.0 * ayylmao[2]) as u32;
            println!("{} {} {}", ir,ig,ib);
        }
    }
}