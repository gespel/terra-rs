use std::fs::File;
use image::{Rgb, RgbImage};
use tiff::decoder::{Decoder, DecodingResult};

pub struct Terra {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<i16>
}

impl Terra {
    pub fn new(file_name: &str) -> Self {
        let file = File::open(file_name).unwrap();
        let mut decoder = Decoder::new(file).unwrap();
        let (width, height) = decoder.dimensions().unwrap();

        let image = decoder.read_image().unwrap();
        match image {
            DecodingResult::I16(pixels) => {
                Terra { 
                    width,
                    height,
                    pixels
                }
            }
            other => {
                panic!("Unerwarteter Pixeltyp: {:?}", other);
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<i16> {
        let idx = (y as usize).checked_mul(self.width as usize)? + x as usize;
        self.pixels.get(idx).copied()
    }

    pub fn render_debug_png(&self, output_path: &str) -> image::ImageResult<()> {
        if self.pixels.is_empty() {
            let empty = RgbImage::new(self.width, self.height);
            return empty.save(output_path);
        }

        let mut min = i16::MAX;
        let mut max = i16::MIN;
        for &v in &self.pixels {
            min = min.min(v);
            max = max.max(v);
        }

        let range = (max as f32 - min as f32).max(1.0);
        let mut img = RgbImage::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize * self.width as usize) + x as usize;
                let v = self.pixels[idx] as f32;
                let t = ((v - min as f32) / range).clamp(0.0, 1.0);

                // simple heatmap: blue -> green -> yellow -> red
                let (r, g, b) = if t < 0.33 {
                    let k = t / 0.33;
                    (0_u8, (k * 255.0) as u8, (255.0 - k * 255.0) as u8)
                } else if t < 0.66 {
                    let k = (t - 0.33) / 0.33;
                    ((k * 255.0) as u8, 255_u8, 0_u8)
                } else {
                    let k = (t - 0.66) / 0.34;
                    (255_u8, (255.0 - k * 255.0) as u8, 0_u8)
                };

                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }

        img.save(output_path)
    }
}