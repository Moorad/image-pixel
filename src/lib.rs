pub mod colors;
pub mod config;
pub mod renderer;
pub mod sprite;
pub mod spriteset;

pub fn render(cfg: &config::Config) {
    renderer::Renderer::from(&cfg)
        .render_image(&cfg)
        .expect("A problem happened while rendering image")
}
