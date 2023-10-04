use std::fs::DirEntry;

use image::{GenericImageView, Pixel};

pub struct Sprite {
    pub file_data: DirEntry,
}

impl Sprite {
    // fn load_from(path: String) -> Sprite {

    // }

    pub fn from(dir_entry: DirEntry) -> Sprite {
        Sprite {
            file_data: dir_entry,
        }
    }

    pub fn average_color(self) -> [u8; 3] {
        let img = image::open(self.file_data.path()).unwrap();
        let dimensions = img.dimensions();

        let mut color_sum = [0u64, 0u64, 0u64];

        for y in 0..dimensions.0 {
            for x in 0..dimensions.1 {
                let pixel = img.get_pixel(x, y);
                let channels = pixel.channels();
                color_sum[0] += channels[0] as u64;
                color_sum[1] += channels[1] as u64;
                color_sum[2] += channels[2] as u64;
            }
        }

        let total_pixels = (dimensions.0 * dimensions.1) as u64;

        let color_sum = [
            (color_sum[0] / total_pixels) as u8,
            (color_sum[1] / total_pixels) as u8,
            (color_sum[2] / total_pixels) as u8,
        ];

        color_sum
    }
}
