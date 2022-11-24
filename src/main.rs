//! An example of generating julia fractals.
use image::{ImageBuffer};
use num_complex::{self, Complex32};
use clap::Parser;

/// Simple program to generate Julia or Mandelbrot fractal images.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(long, short, default_value="", hide_default_value=true, help="Write Julia fractal image to the specifed file.")]
   julia: String,

   #[arg(long,default_value_t=-0.4,visible_alias="jr", env="JULIA_RE")]
   julia_c_real: f32,

   #[arg(long,default_value_t=0.6,visible_alias="ji", env="JULIA_IM")]
   julia_c_imaginary: f32,

   #[arg(long,default_value_t=0.0,visible_alias="cr", env="CENTER_X")]
   center_real: f32,

   #[arg(long,default_value_t=0.0,visible_alias="ci", env="CENTER_Y")]
   center_imaginary: f32,

   #[arg(long,short,default_value_t=3.0, env="SCALE")]
   scale: f32,

   #[arg(long,default_value_t=800,env="FRACTAL_WIDTH", help="Image WIDTH in pixels.")]
   width: u32,

   #[arg(long,default_value_t=800,env="FRACTAL_HEIGHT", help="Image HEIGHT in pixels.")]
   height: u32,

   #[arg(long, short, default_value="", hide_default_value=true, help="Write Mandlebrot fractal image to the specified file.")]
   mandlebrot: String,
}

struct FractalImage {
    img: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    center: Complex32,
    scale: Complex32,
}


fn julia(frac: &mut FractalImage,  c: Complex32) {
    let bg_scale_x = 255.0 / (frac.img.width() as f32);
    let bg_scale_y = 255.0 / (frac.img.height() as f32);

    let x_offset = (frac.img.width() as f32 )* frac.scale.re / 2.0 - frac.center.re;
    let y_offset = (frac.img.height() as f32 )* frac.scale.im / 2.0 - frac.center.im;

    for x in 0..frac.img.width() {
        let r = ((x as f32) * bg_scale_x )as u8;
        let x0 = (x as f32) * frac.scale.im - x_offset;

        for y in 0..frac.img.height() {
            let b = ((y as f32) * bg_scale_y) as u8;
            let y0 = (y as f32) * frac.scale.re - y_offset;
            let mut x2 = x0*x0;
            let mut y2 = y0*y0;
            let mut w = (x0 + y0) * (x0 + y0);
            let mut i = 0;
            while i < 255 && x2 + y2 <= 4.0 {
                let x = x2 - y2 + c.re;
                let y = w - x2 - y2 + c.im;
                let xy = x + y;
                x2 = x * x;
                y2 = y * y;
                w = xy * xy;
                i += 1;
            }

            let pixel = frac.img.get_pixel_mut(x, y);
            *pixel = image::Rgb([r/2 + i/2, i as u8, b/2 + i/2]);
        }
    }}

fn mandlebrot(frac: &mut FractalImage) {
    let bg_scale_x = 255.0 / (frac.img.width() as f32);
    let bg_scale_y = 255.0 / (frac.img.height() as f32);

    let x_offset = (frac.img.width() as f32 )* frac.scale.re / 2.0 - frac.center.re;
    let y_offset = (frac.img.height() as f32 )* frac.scale.im / 2.0 - frac.center.im;

    for x in 0..frac.img.width() {
        let r = ((x as f32) * bg_scale_x )as u8;
        let x0 = (x as f32) * frac.scale.im - x_offset;

        for y in 0..frac.img.height() {
            let b = ((y as f32) * bg_scale_y) as u8;
            let y0 = (y as f32) * frac.scale.re - y_offset;
            let mut x2 = 0.0f32;
            let mut y2 = x2;
            let mut w = x2;
            let mut i = 0;
            while i < 255 && x2 + y2 <= 4.0 {
                let x = x2 - y2 + x0;
                let y = w - x2 - y2 + y0;
                let xy = x + y;
                x2 = x * x;
                y2 = y * y;
                w = xy * xy;
                i += 1;
            }

            let pixel = frac.img.get_pixel_mut(x, y);
            *pixel = image::Rgb([r/2 + i/2, i as u8, b/2 + i/2]);
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.julia != "" {
        let mut j = FractalImage {
            img: ImageBuffer::new(args.width, args.height),
            center: Complex32{re: args.center_real, im: args.center_imaginary},
            scale: Complex32{ re: args.scale / (args.width as f32), im: args.scale / (args.height as f32) },
        };

        julia(&mut j, Complex32{re: args.julia_c_real, im: args.julia_c_imaginary});
        j.img.save(args.julia).unwrap();
    }

    if args.mandlebrot != "" {
        let mut m = FractalImage {
            img: ImageBuffer::new(args.width, args.height),
            center: Complex32{re: args.center_real, im: args.center_imaginary},
            scale: Complex32{ re: args.scale / (args.width as f32), im: args.scale / (args.height as f32) },
        };
        mandlebrot(&mut m);
        m.img.save(args.mandlebrot).unwrap();
    }
}