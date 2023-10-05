use std::{collections::HashMap, fs};

use crate::{config::Config, sprite::Sprite};

pub struct SpriteSet<'a> {
    set_name: &'a String,
    pub sprites: Vec<Sprite>,
}

impl SpriteSet<'_> {
    pub fn new(cfg: &Config) -> SpriteSet {
        let mut ss = SpriteSet {
            set_name: &cfg.sprite_set_name,
            sprites: Vec::new(),
        };

        ss.load_sprites(cfg.pixel_size);

        return ss;
    }

    pub fn load_sprites(&mut self, size: u32) {
        let files: fs::ReadDir = self
            .list_files()
            .expect("There was a problem while reading the sprite set: {:?}");

        for f in files {
            self.sprites.push(Sprite::from(
                f.expect("There was a problem while reading the sprite file metadata: {:?}"),
                size,
            ))
        }
    }

    pub fn get_image_color_mapping(&self) -> HashMap<String, [u8; 3]> {
        let mut image_color_mapping: HashMap<String, [u8; 3]> = HashMap::new();

        for sprite in &self.sprites {
            let file_name = sprite.file_data.file_name().into_string().unwrap();
            let average_color = sprite.average_color();
            image_color_mapping.insert(file_name, average_color);
        }

        return image_color_mapping;
    }

    pub fn get_color_image_mapping(self) -> HashMap<[u8; 3], String> {
        let mut color_image_mapping: HashMap<[u8; 3], String> = HashMap::new();

        for sprite in self.sprites {
            let file_name = sprite.file_data.file_name().into_string().unwrap();
            let average_color = sprite.average_color();
            color_image_mapping.insert(average_color, file_name);
        }

        return color_image_mapping;
    }

    pub fn list_files(&self) -> Result<fs::ReadDir, std::io::Error> {
        let sn = self.set_name;
        fs::read_dir(format!("./sprites/{sn}"))
    }

    pub fn get_sprite(&self, sprite_name: String) -> Option<&Sprite> {
        for sprite in &self.sprites {
            if sprite.file_data.file_name().into_string().unwrap() == sprite_name {
                return Some(&sprite);
            }
        }

        return None;
    }
}
