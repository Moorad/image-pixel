use clap::Parser;
use imgpx::{
    config::Config,
    progdata::{self},
    render,
};

fn main() {
    let cfg: Config = Config::parse();

    progdata::init(&cfg)
	.expect("Unable to create program data folder at HOME directory (Linux) or APPDATA folder (Windows), {:?}");

    let sprite_set_path = match cfg.zip_sprite_set {
        true => progdata::zip::unzip(&cfg.sprite_set_path).unwrap(),
        false => cfg.sprite_set_path.clone(),
    };

    render(&sprite_set_path, &cfg);

    progdata::tear_down().expect("Unable to clean up program data, {:?}");
}
