use image::{imageops::FilterType::Nearest, GenericImageView, ImageError, Pixel};

use crate::{colors, config::Config, spriteset::SpriteSet};

pub struct Render<'a> {
    input_image: image::DynamicImage,
    sprite_set: SpriteSet<'a>,
}

impl Render<'_> {
    pub fn from<'a>(input_file: &'a String, sprite_set: SpriteSet<'a>) -> Render<'a> {
        let input_image = image::open(input_file).expect("Could not load input image");

        Render {
            input_image: input_image,
            sprite_set: sprite_set,
        }
    }

    pub fn render_image(self, cfg: &Config) -> Result<(), ImageError> {
        let dimensions = self.input_image.dimensions();
        let calculated_height =
            (dimensions.1 as f32 * (cfg.img_pixel_width as f32 / dimensions.0 as f32)) as u32;

        let image_color_mapping = self.sprite_set.get_image_color_mapping();
        let intermediate_img =
            self.input_image
                .resize(cfg.img_pixel_width, calculated_height, Nearest);

        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(
            cfg.img_pixel_width * cfg.pixel_size,
            calculated_height * cfg.pixel_size,
        );

        for x in 0..intermediate_img.dimensions().0 {
            for y in 0..intermediate_img.dimensions().1 {
                let pixel = intermediate_img.get_pixel(x, y);
                let mut min_dist_img = "";
                let mut min_dist = f32::MAX;

                for img in image_color_mapping.keys() {
                    let distance = colors::color_distance(
                        &[pixel[0], pixel[1], pixel[2]],
                        image_color_mapping.get(img).unwrap(),
                    );

                    if distance < min_dist {
                        min_dist_img = img;
                        min_dist = distance;
                    }
                }

                let sprite = self
                    .sprite_set
                    .get_sprite(min_dist_img.to_string())
                    .unwrap();

                for sx in 0..sprite.image.dimensions().0 {
                    for sy in 0..sprite.image.dimensions().1 {
                        imgbuf.put_pixel(
                            (x * cfg.pixel_size) + sx,
                            (y * cfg.pixel_size) + sy,
                            sprite.image.get_pixel(sx, sy).to_rgb(),
                        )
                    }
                }
            }
        }

        imgbuf.save(&cfg.output_dest)
    }
}
