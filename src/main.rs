use indicatif::{ProgressBar, ProgressStyle};
mod vec3;
use vec3::Vec3;
fn main() {
    //image width & heiight
    //punish CPU w/32k :troll: ?
    //let width = 30720;
    //let height = 17280;

    let width = 1080;
    let height = 720;

    // Create a new progress bar with 100 steps
    let pb: ProgressBar = ProgressBar::new(height);

    // Set the style of the progress bar
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    //Render
    //P3 => PPM file format
    println!("P3\n{} {}\n255", width, height);

    // ".." = from a to b (excluding b)
    for i in 0..height {
        for j in 0..width {
            //Rust doesn't allow float / int division so we need to cast to f64 (for type saftey I guess)
            let r: f64 = i as f64 / (height - 1) as f64;
            let g: f64 = j as f64 / (width - 1) as f64;
            let b: f64 = 0.0;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
        pb.inc(1);
    }
    pb.finish();
}
