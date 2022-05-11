mod camera;
mod material;
mod object;
mod ray;
mod vector;

use image::RgbImage;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle};
use material::{Dielectric, Lambertian, Metal};
use object::{Scene, Sphere};
use rayon::prelude::*;
use vector::{Point, Vec3};

fn main() {
    //image
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples: u32 = 100;
    let max_depth = 50;

    //camera struct
    let camera = camera::Camera::default();

    //create image buffer
    let mut buffer = RgbImage::new(img_width, img_height);

    //world
    let objects: Scene = vec![
        Box::new(Sphere::new(
            //center
            v!(0, 0, -1),
            0.5,
            Lambertian::new(v!(0.7, 0.3, 0.3)),
        )),
        Box::new(Sphere::new(
            //ground
            v!(0, -100.5, -1),
            100.0,
            Lambertian::new(v!(0.8, 0.8, 0.0)),
        )),
        Box::new(Sphere::new(
            //left
            v!(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(1.5),
        )),
        Box::new(Sphere::new(
            //right
            v!(1.0, 0.0, -1.0),
            0.5,
            Metal::new(v!(0.8, 0.6, 0.2), 1.0),
        )),
    ];

    println!("Rendering Scene...");
    let bar = ProgressBar::new((img_width * img_height) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
            )
            .progress_chars("#>-")
            .on_finish(ProgressFinish::WithMessage("-- Done!".into())),
    );

    buffer
        .enumerate_pixels_mut()
        .par_bridge() // Rayon go brrrrrrr
        .progress_with(bar) // Indicatif go brrrrrr
        .for_each(|(i, j, px)| {
            //pixel coordinates as scalars from 0.0 <= t <= 1.0
            //add a little randomness for antialiasing
            let mut colour = v!(0);
            for _ in 0..samples {
                let u = (i as f64 + rand::random::<f64>()) / (img_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (img_height - 1) as f64;

                //get the ray from the camera and then colour it
                let ray = camera.get_ray(u, v);
                colour = colour + ray::colour(&objects, &ray, max_depth);
            }
            //save pixel colour to buffer
            *px = (colour / (samples as f64)).to_rgb();
        });
    buffer.save("render.png").expect("Could not save image");
}
