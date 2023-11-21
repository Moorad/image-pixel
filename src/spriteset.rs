use std::{
    collections::HashMap,
    fs,
    io::{stdout, Write},
};

use crate::{progdata::cache, sprite::Sprite};

pub struct SpriteSet<'a> {
    set_path: &'a str,
    pub sprites: Vec<Sprite>,
}

impl SpriteSet<'_> {
    pub fn new(sprite_set_path: &str, pixel_size: u32) -> SpriteSet {
        let mut ss = SpriteSet {
            set_path: sprite_set_path,
            sprites: Vec::new(),
        };

        ss.load_sprites(pixel_size);

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

    pub fn get_image_color_mapping(&self, disable_cache: bool) -> HashMap<String, [u8; 3]> {
        if !disable_cache {
            let set_name = &self.set_path.split("/").last().unwrap();
            let cache_exists = cache::validate(&set_name);

            if cache_exists {
                return cache::load(&set_name).expect("Could not read cache {:?}");
            }
        }

        let mut image_color_mapping: HashMap<String, [u8; 3]> = HashMap::new();
        let mut stdout = stdout();
        for sprite in &self.sprites {
            let file_name = sprite.file_data.file_name().into_string().unwrap();
            let average_color = sprite.average_color();
            image_color_mapping.insert(file_name, average_color);

            stdout
                .write(
                    format!(
                        "\r🎨 Calculating colour averages: {}% ({}/{})",
                        ((image_color_mapping.len() as f32 / self.sprites.len() as f32) * 100.0)
                            .floor(),
                        image_color_mapping.len(),
                        self.sprites.len()
                    )
                    .as_bytes(),
                )
                .unwrap();

            stdout.flush().unwrap();
        }

        println!();

        if !disable_cache {
            let set_name = &self.set_path.split("/").last().unwrap();
            cache::save(&set_name, &image_color_mapping).expect("Could not write cache {:?}");
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
        fs::read_dir(self.set_path)
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
