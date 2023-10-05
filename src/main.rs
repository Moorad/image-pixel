use std::env;

pub mod colors;
pub mod render;
pub mod sprite;
pub mod spriteset;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Missing arguments. Please use cargo run -- [input_file] [sprite_set]");
        return;
    }

    let input_file = &args[1];
    let set_name = &args[2];
    dbg!(input_file, set_name);

    let sprite_set = spriteset::SpriteSet::new(set_name);

    // let image_color_mapping = sprite_set.get_image_color_mapping();

    // let files = sprite_set
    //     .list_files()
    //     .expect("There was a problem while reading the sprite set: {:?}");

    // let mut image_color_mapping: HashMap<String, [u8; 3]> = HashMap::new();

    // for f in files {
    //     let sprite = sprite::Sprite::from(f.unwrap());
    //     let file_name = sprite.file_data.file_name().into_string().unwrap();
    //     let average_color = sprite.average_color();
    //     image_color_mapping.insert(file_name, average_color);
    // }

    // dbg!(image_color_mapping);

    let render = render::Render::from(input_file, sprite_set);

    render
        .render_image("./rendered_image.png".to_string())
        .expect("A problem happened while rendering image")

    // dbg!(image_color_mapping.get("sand.png").unwrap());
}
