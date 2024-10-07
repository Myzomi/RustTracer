mod vec3;
mod color;
mod ray;
use indicatif::{ProgressBar, ProgressStyle};
use color::*;
use vec3::*;
use ray::*;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();
    let a = r.direction().length_squared();
    let b = -2.0 * dot(&oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n: Vec3 = unit_vector(&(r.at(t) - Point3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction: Vec3 = unit_vector(&r.direction());
    let a = 0.5*(unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}


fn main() {
    //image width & heiight
    //punish CPU w/32k :troll: ?
    //let width = 30720;
    //let height = 17280;

    let aspect_ratio = 16.0 / 9.0;
    let width = 1080;

    //Calculate image height & ensure it's at least 1
    let height: u64 = ((width as f64 / aspect_ratio) as u64).max(1);

    let pb: ProgressBar = ProgressBar::new(height);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    //Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    //viewport_height is negative because we are looking down the negative z-axis
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    //convert h and v vectors to be pixel coordinates
    let pixel_delta_u = viewport_u / width as f64;
    let pixel_delta_v = viewport_v / height as f64;

    //Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    //Render
    //P3 => PPM Color file format
    println!("P3\n{} {}\n255", width, height);

    // ".." = from a to b (excluding b)
    for i in 0..height {
        for j in 0..width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_v) + (j as f64 * pixel_delta_u);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            
            let pixel_color: Color = ray_color( &r) ;
            write_color(&pixel_color);
        }
        pb.inc(1);
    }
    pb.finish();
}