mod vec3;
mod color;
use indicatif::{ProgressBar, ProgressStyle};
use color::*;
use vec3::*;

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
    //P3 => PPM Color file format
    println!("P3\n{} {}\n255", width, height);

    // ".." = from a to b (excluding b)
    for i in 0..height {
        for j in 0..width {
            //Rust doesn't allow float / int division so we need to cast to f64 (for type saftey I guess)
            let pixel_color: Color = Color::new(i as f64 / (width - 1) as f64, j as f64 / (height - 1) as f64, 0.0 as f64);
            write_color(&pixel_color);
        }
        pb.inc(1);
    }
    pb.finish();
}
