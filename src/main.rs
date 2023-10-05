use clap::Parser;

pub mod colors;
pub mod config;
pub mod render;
pub mod sprite;
pub mod spriteset;

fn main() {
    let cfg: config::Config = config::Config::parse();

    let sprite_set = spriteset::SpriteSet::new(&cfg);

    let render = render::Render::from(&cfg.input_file, sprite_set);

    render
        .render_image(&cfg)
        .expect("A problem happened while rendering image")

    // dbg!(image_color_mapping.get("sand.png").unwrap());
}
