use clap::Parser;
use imgpx::{
    config::Config,
    render,
};

fn main() {
    let cfg: Config = Config::parse();

    render(&cfg);
}
