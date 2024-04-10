use image::{load_from_memory_with_format, RgbaImage};
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;

use include_dir::{include_dir, Dir, DirEntry};

static mut SAMPLES: OnceLock<Vec<ColorSample>> = OnceLock::new();
static SAMPLES_DIR: Dir = include_dir!("src/samples");

pub fn read_jpeg_bytes() {
    unsafe {
        SAMPLES.get_or_init(|| {
            log("reading image samples");
            read_color_samples().unwrap()
        });
    }
}

pub fn get_closest_color<'a>(r: u8, g: u8, b: u8) -> &'static ColorSample {
    unsafe {
        let color_samples = SAMPLES.get_mut().unwrap();

        let mut closest = None;
        let mut min_diff: f32 = 4294967295.0; //0xFFFFFFFF
        for sample in color_samples {
            let diff = get_distance(sample.r, sample.g, sample.b, r, g, b);
            if diff < min_diff {
                closest = Some(sample);
                min_diff = diff;
            }
        }

        let closest = closest.unwrap();
        if closest.image.is_none() {
            let sample_image =
                load_from_memory_with_format(closest.raw_bytes, image::ImageFormat::Jpeg)
                    .unwrap()
                    .to_rgba8();
            closest.image = Some(sample_image);
        }
        closest
    }
}

/// returns squared euclidian color distance
/// as if colors were points in 3d space
fn get_distance(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> f32 {
    let red_dif = r1 as f32 - r2 as f32;
    let green_dif = g1 as f32 - g2 as f32;
    let blue_dif = b1 as f32 - b2 as f32;
    return red_dif * red_dif + green_dif * green_dif + blue_dif * blue_dif;
}

/// read all sample jpegs into memory
pub fn read_color_samples() -> anyhow::Result<Vec<ColorSample>> {
    let mut color_samples: Vec<ColorSample> = Vec::new();

    for entry in SAMPLES_DIR.entries() {
        if let DirEntry::File(f) = entry {
            let filename = entry.path().file_name().unwrap().to_str().unwrap();
            let hex_r = &filename[0..2];
            let hex_g = &filename[2..4];
            let hex_b = &filename[4..6];

            color_samples.push(ColorSample {
                filename: filename.into(),
                r: u8::from_str_radix(&hex_r, 16)?,
                g: u8::from_str_radix(&hex_g, 16)?,
                b: u8::from_str_radix(&hex_b, 16)?,
                raw_bytes: f.contents(),
                image: None,
            });
        }
    }
    log("Done reading image samples");
    Ok(color_samples)
}

#[derive(Debug)]
pub struct ColorSample {
    pub filename: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub raw_bytes: &'static [u8],
    pub image: Option<RgbaImage>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
