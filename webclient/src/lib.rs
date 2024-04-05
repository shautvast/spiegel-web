use image::{GenericImage, GenericImageView, Pixel, Rgba, RgbaImage};
use photon_rs::PhotonImage;
use std::collections::LinkedList;
mod quantizer;
mod samples;

use samples::log;
use wasm_bindgen::prelude::*;
static BLACK: Rgba<u8> = Rgba([0, 0, 0, 0]);

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
    let width = photon_image.get_width();
    let height = photon_image.get_height();

    if width == 0 || height == 0 {
        return;
    }
    samples::init();
    let raw_pixels = photon_image.get_raw_pixels().to_vec(); //argh!, slice should work but doesn't
    let out = RgbaImage::from_vec(width, height, raw_pixels).unwrap();
    log(&format!("stort"));
    // let out = imageproc::filter::gaussian_blur_f32(&rs_image, 3.0);
    // log(&format!("gaussian done"));
    // let mut out = quantizer::quantize(&out, 256);
    let mut out = imageproc::filter::median_filter(&out, median_kernelsize, median_kernelsize);
    //
    log(&format!("median done"));
    let out = apply_samples_to_image(&mut out);
    log(&format!("applying done"));
    *photon_image = PhotonImage::new(out.into_raw(), width, height);
}

fn apply_samples_to_image(src: &mut RgbaImage) -> RgbaImage {
    let mut out = RgbaImage::new(src.width(), src.height());
    unsafe {
        for y in 0..src.height() {
            for x in 0..src.width() {
                if out.unsafe_get_pixel(x, y) == BLACK {
                    let pixel = src.unsafe_get_pixel(x, y);
                    if pixel != BLACK {
                        let sample = samples::get_closest_color(pixel[0], pixel[1], pixel[2])
                            .image
                            .as_ref()
                            .unwrap();
                        fill(src, sample, &mut out, pixel, x, y);
                    }
                }
            }
        }
    }
    out
}

fn fill(
    src: &mut RgbaImage,
    sample: &RgbaImage,
    dest: &mut RgbaImage,
    color: Rgba<u8>,
    px: u32,
    py: u32,
) {
    unsafe {
        let height = sample.height();
        let width = sample.width();
        let mut points = LinkedList::new();
        if is_same(src.unsafe_get_pixel(px, py), color) {
            points.push_back(Coord(px, py));
        }

        while !points.is_empty() {
            if let Some(coord) = points.pop_back() {
                let orig_pixel = src.unsafe_get_pixel(coord.0, coord.1);
                let x = coord.0;
                let y = coord.1;
                if src.unsafe_get_pixel(x, y) != BLACK {
                    if is_same(orig_pixel, color) {
                        let mut xx = x;
                        let mut yy = y;
                        while xx >= width {
                            xx -= width;
                        }
                        while yy >= height {
                            yy -= height;
                        }
                        dest.unsafe_put_pixel(x, y, sample.unsafe_get_pixel(xx, yy));
                        src.unsafe_put_pixel(x, y, BLACK);
                        if x > 1 {
                            points.push_front(Coord(x - 1, y));
                        }
                        if y > 1 {
                            points.push_front(Coord(x, y - 1));
                        }
                        if x < src.width() - 1 {
                            points.push_front(Coord(x + 1, y));
                        }
                        if y < src.height() - 1 {
                            points.push_front(Coord(x, y + 1));
                        }
                    }
                }
            } else {
                println!("break");
                break;
            }
        }
    }
}

fn is_same(p1: Rgba<u8>, p2: Rgba<u8>) -> bool {
    let p1 = p1.channels();
    let p2 = p2.channels();
    i16::abs(p1[0] as i16 - p2[0] as i16) < 4
        && i16::abs(p1[1] as i16 - p2[1] as i16) < 4
        && i16::abs(p1[2] as i16 - p2[2] as i16) < 4
}

#[derive(Debug)]
struct Coord(u32, u32);
