use std::{
    fs,
    io::{stdout, Write},
};

use image::{imageops::FilterType::Nearest, GenericImageView, Pixel};

use crate::{colors, config::Config, spriteset::SpriteSet};

pub struct Renderer<'a> {
    input_image: image::DynamicImage,
    sprite_set: SpriteSet<'a>,
}

impl Renderer<'_> {
    pub fn from<'a>(
        input_file: &'a str,
        sprite_set_path: &'a str,
        pixel_size: u32,
    ) -> Renderer<'a> {
        let input_image = image::open(input_file).expect("Could not load input image");

        let sprite_set = SpriteSet::new(sprite_set_path, pixel_size);

        Renderer {
            input_image,
            sprite_set,
        }
    }

    pub fn render_image(self, cfg: &Config) -> Result<(), image::ImageError> {
        let mut stdout = stdout();

        let calculated_height = (self.input_image.height() as f32
            * (cfg.img_pixel_width as f32 / self.input_image.width() as f32))
            as u32;

        let image_color_mapping = self.sprite_set.get_image_color_mapping(cfg.disable_caching);
        let scaled_down_img =
            self.input_image
                .resize(cfg.img_pixel_width, calculated_height, Nearest);

        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(
            cfg.img_pixel_width * cfg.pixel_size,
            calculated_height * cfg.pixel_size,
        );

        for x in 0..scaled_down_img.width() {
            for y in 0..scaled_down_img.height() {
                let pixel = scaled_down_img.get_pixel(x, y);
                let mut min_dist_img = &String::new();
                let mut min_dist = f32::MAX;

                for (img_name, color) in &image_color_mapping {
                    let distance = colors::color_distance(&[pixel[0], pixel[1], pixel[2]], color);

                    if distance < min_dist {
                        min_dist_img = img_name;
                        min_dist = distance;
                    }
                }

                let sprite = self
                    .sprite_set
                    .get_sprite(min_dist_img.to_string())
                    .unwrap();

                let offset_x: u32 = x * cfg.pixel_size;
                let offset_y = y * cfg.pixel_size;

                for sx in 0..sprite.image.dimensions().0 {
                    for sy in 0..sprite.image.dimensions().1 {
                        imgbuf.put_pixel(
                            offset_x + sx,
                            offset_y + sy,
                            sprite.image.get_pixel(sx, sy).to_rgb(),
                        )
                    }
                }
            }

            let processed_pixels = x as f32 * (scaled_down_img.height() as f32 - 1.0);
            let total_pixels =
                (scaled_down_img.width() as f32 - 1.0) * (scaled_down_img.height() as f32 - 1.0);

            stdout
                .write(
                    format!(
                        "\r🖼️ Processing image: {}% ({}/{})",
                        ((processed_pixels / total_pixels) * 100.0).floor(),
                        processed_pixels,
                        total_pixels
                    )
                    .as_bytes(),
                )
                .unwrap();

            stdout.flush().unwrap();
        }

        println!();

        println!(
            "💾 Saving image: {:?}",
            fs::canonicalize(&cfg.output_dest).unwrap()
        );
        imgbuf.save(&cfg.output_dest)
    }
}
