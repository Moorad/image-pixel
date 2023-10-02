use std::env;
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

    for path in files {
        dbg!(path.unwrap().file_type());
    }

    println!("Hello, world!");
}
