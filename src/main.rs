use clap::Parser;
use image_pixel::{config::Config, render};

fn main() {
    let cfg: Config = Config::parse();

    render(&cfg);
}
