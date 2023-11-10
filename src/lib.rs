pub mod colors;
pub mod config;
pub mod progdata;
pub mod renderer;
pub mod sprite;
pub mod spriteset;

pub fn render(sprite_set_path: &str, cfg: &config::Config) {
    renderer::Renderer::from(&cfg.input_file, sprite_set_path, cfg.pixel_size)
        .render_image(&cfg)
        .expect("A problem happened while rendering image")
}
