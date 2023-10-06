use clap::Parser;
use imgpx::{
    config::Config,
    progdata::{self},
    render,
};

fn main() {
    let cfg: Config = Config::parse();

    progdata::init().expect("Unable to create program data folder at HOME directory (Linux) or APPDATA folder (Windows), {:?}");

    render(&cfg);
}
