use image::{GenericImage, GenericImageView, Pixel, Rgba, RgbaImage};
use photon_rs::PhotonImage;
use std::collections::LinkedList;
use image::imageops::FilterType;

mod samples;

use samples::log;
use wasm_bindgen::prelude::*;

static BLACK: Rgba<u8> = Rgba([0, 0, 0, 0]);

#[wasm_bindgen]
pub fn spiegel(photon_image: PhotonImage, median_kernelsize: u32, preview: bool) -> PhotonImage {
    samples::read_jpeg_bytes();

    let width = photon_image.get_width();
    let height = photon_image.get_height();

    let raw_pixels = photon_image.get_raw_pixels().to_vec();
    let i1 = RgbaImage::from_vec(width, height, raw_pixels).unwrap();

    let i2 = if preview {
        image::imageops::resize(&i1, u32::min(500, width >> 1), u32::min(500, height >> 1), FilterType::Nearest)
    } else {
        image::imageops::resize(&i1, u32::min(500, width), u32::min(500, height), FilterType::Nearest)
    };
    let mut i3 = imageproc::filter::median_filter(&i2, median_kernelsize, median_kernelsize);
    let i4 = if !preview {
        apply_samples_to_image(&mut i3)
    } else {
        i3
    };
    let i5 = image::imageops::resize(&i4, width, height, FilterType::Nearest);
    PhotonImage::new(i5.into_raw(), width, height)
}

fn apply_samples_to_image(src: &mut RgbaImage) -> RgbaImage {
    let mut out = RgbaImage::new(src.width(), src.height());
    unsafe {
        for y in 0..src.height() {
            log(&format!("{}", y));
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
