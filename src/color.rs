use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) -> () {
    //Rust doesn't allow float / int division so we need to cast to f64 (for type saftey I guess)
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    //Convert each f64 [0,1] value to byte range [0,225]
    let ir: u8 = (255.999 * r) as u8;
    let ig: u8 = (255.999 * g) as u8;
    let ib: u8 = (255.999 * b) as u8;

    println!("{} {} {}", ir, ig, ib);
}