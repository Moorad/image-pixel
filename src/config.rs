use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "imgpx", author = "Moorad")]
pub struct Config {
    pub input_file: String,
    pub sprite_set_name: String,
    #[arg(
        short = 'o',
        long = "out",
        value_name = "FILE",
        default_value = "./out.png"
    )]
    pub output_dest: String,
    #[arg(short, long = "pixel-size", default_value = "16")]
    pub pixel_size: u32,
    #[arg(short, long = "size", default_value = "128")]
    pub img_pixel_width: u32,
}
