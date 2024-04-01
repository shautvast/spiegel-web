use anyhow::Error;
use futures::executor::block_on;
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use photon_rs::PhotonImage;
use std::collections::HashSet;
use std::io::Cursor;
mod quantizer;
mod samples;

use wasm_bindgen::prelude::*;

/// Apply a median filter
///
///  # Arguments
/// * `img` - A PhotonImage.
/// * `x_radius` - x radius of median window
/// * `y_radius` - y radius of median window
#[wasm_bindgen]
pub fn median(photon_image: &mut PhotonImage, x_radius: u32, y_radius: u32) {
    let width = photon_image.get_width();
    let height = photon_image.get_height();

    if width == 0 || height == 0 {
        return;
    }

    let raw_pixels = photon_image.get_raw_pixels().to_vec(); //argh!, slice should work but doesn't
    let rs_image = RgbaImage::from_vec(width, height, raw_pixels).unwrap();

    let filtered: Vec<u8> =
        imageproc::filter::median_filter(&rs_image, x_radius, y_radius).into_raw();

    *photon_image = PhotonImage::new(filtered, width, height);
}

#[wasm_bindgen]
pub fn spiegel(photon_image: &mut PhotonImage, median_kernelsize: u32) {
    // let width = photon_image.get_width();
    // let height = photon_image.get_height();

    // let raw_pixels = photon_image.get_raw_pixels().to_vec(); //argh!, slice should work but doesn't
    // let rs_image = RgbImage::from_vec(width, height, raw_pixels).unwrap();

    // // // // println!("applying gaussian blur filter");
    // // let gauss = imageproc::filter::gaussian_blur_f32(rs_image, 4.0);
    // // // println!("applying median filter");
    // let median = imageproc::filter::median_filter(&rs_image, median_kernelsize, median_kernelsize)
    //     .into_raw();
    // // // println!("applying color quantization filter");
    // // // let quantized = quantizer::quantize(&median, 256);

    // // // println!("applying samples");
    // // let out = block_on(apply_samples_to_image(&mut median));

    // *photon_image = PhotonImage::new(median, width, height);
    //
    let width = photon_image.get_width();
    let height = photon_image.get_height();

    if width == 0 || height == 0 {
        return;
    }

    let raw_pixels = photon_image.get_raw_pixels().to_vec(); //argh!, slice should work but doesn't
    let rs_image = RgbaImage::from_vec(width, height, raw_pixels).unwrap();

    let mut filtered =
        imageproc::filter::median_filter(&rs_image, median_kernelsize, median_kernelsize);
    let out = block_on(apply_samples_to_image(&mut filtered));

    *photon_image = PhotonImage::new(out.into_raw(), width, height);
}

async fn apply_samples_to_image(src: &mut RgbaImage) -> RgbaImage {
    log("applying");
    let mut imgbuf = RgbaImage::new(src.width(), src.height());
    unsafe {
        for y in 0..src.height() {
            for x in 0..src.width() {
                let pixel = &src.unsafe_get_pixel(x, y);
                if imgbuf.unsafe_get_pixel(x, y).channels() == [0, 0, 0] {
                    if let Ok(sample) = get_image(pixel).await {
                        fill(src, sample, &mut imgbuf, pixel, x, y);
                    }
                }
            }
        }
    }
    imgbuf
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

async fn get_image(pixel: &Rgba<u8>) -> anyhow::Result<RgbaImage, Error> {
    let rgb = format!("{:02X?}{:02X?}{:02X?}", pixel[0], pixel[1], pixel[2]);
    log(&format!("get {}", rgb));

    let bytes = reqwest_wasm::get(format!("/api/color/{}", rgb))
        .await?
        .bytes()
        .await?
        .to_vec();

    Ok(ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()?
        .as_rgba8()
        .unwrap()
        .clone())

    // should probably cache it
}

fn fill(
    src: &mut RgbaImage,
    sample: RgbaImage,
    dest: &mut RgbaImage,
    color: &Rgba<u8>,
    px: u32,
    py: u32,
) {
    if color.channels() == [0, 0, 0, 0] {
        return;
    }
    let height = sample.height();
    let width = sample.width();
    let mut points = List::new();
    if is_same(src.get_pixel(px, py), &color) {
        points.push(Point { x: px, y: py });
    }

    while !points.is_empty() {
        if let Some(point) = points.pop() {
            let orig_pixel = src.get_pixel(point.x, point.y);
            let x = point.x;
            let y = point.y;
            if src.get_pixel(x, y).channels() != [0, 0, 0] {
                if is_same(orig_pixel, &color) {
                    let mut xx = x;
                    let mut yy = y;
                    while xx >= width {
                        xx -= width;
                    }
                    while yy >= height {
                        yy -= height;
                    }
                    dest.put_pixel(x, y, *sample.get_pixel(xx, yy));
                    src.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                    if x > 1 {
                        points.push(Point::new(x - 1, y));
                    }
                    if y > 1 {
                        points.push(Point::new(x, y - 1));
                    }
                    if x < src.width() - 1 {
                        points.push(Point::new(x + 1, y));
                    }
                    if y < src.height() - 1 {
                        points.push(Point::new(x, y + 1));
                    }
                }
            }
        } else {
            println!("break");
            break;
        }
    }
}

fn is_same(p1: &Rgba<u8>, p2: &Rgba<u8>) -> bool {
    let p1 = p1.channels();
    let p2 = p2.channels();
    i16::abs(p1[0] as i16 - p2[0] as i16) < 4
        && i16::abs(p1[1] as i16 - p2[1] as i16) < 4
        && i16::abs(p1[2] as i16 - p2[2] as i16) < 4
}

fn get_distance(r: u8, g: u8, b: u8, c2: &Rgba<u8>) -> f32 {
    let red_dif = r as f32 - c2.channels()[0] as f32;
    let green_dif = g as f32 - c2.channels()[1] as f32;
    let blue_dif = b as f32 - c2.channels()[2] as f32;
    // ignore alpha channel
    return f32::sqrt(red_dif * red_dif + green_dif * green_dif + blue_dif * blue_dif);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

struct ColorSample {
    r: u8,
    g: u8,
    b: u8,
    image: RgbImage,
}

#[derive(Debug)]
struct List {
    head: Option<Box<Node>>,
}

impl List {
    fn new() -> Self {
        Self { head: None }
    }
    fn push(&mut self, point: Point<u32>) {
        let new_node = Box::new(Node {
            value: point,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<Point<u32>> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[derive(Debug)]
struct Node {
    value: Point<u32>,
    next: Option<Box<Node>>,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126,
            125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125,
            132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110,
            255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255,
        ];

        let photon_image = PhotonImage::new(raw_pix, 4, 4);

        let colors = determine_colors(&photon_image);
        println!("{:?}", colors);
    }
}
