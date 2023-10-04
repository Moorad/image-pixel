use std::{collections::HashMap, env};

pub mod colors;
pub mod sprite;
pub mod spriteset;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    let set_name = &args[2];
    dbg!(input_file, set_name);

    let sprite_set = spriteset::SpriteSet::new(set_name);

    let files = sprite_set
        .list_files()
        .expect("There was a problem while reading the sprite set: {:?}");

    let mut image_color_mapping: HashMap<String, [u8; 3]> = HashMap::new();
    for f in files {
        let sprite = sprite::Sprite::from(f.unwrap());
        let file_name = sprite.file_data.file_name().into_string().unwrap();
        let average_color = sprite.average_color();
        image_color_mapping.insert(file_name, average_color);
    }

    dbg!(image_color_mapping);
}
