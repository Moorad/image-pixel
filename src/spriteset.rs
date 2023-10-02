use std::fs;

pub struct SpriteSet<'a> {
    set_name: &'a String,
}

impl SpriteSet<'_> {
    pub fn new(set_name: &String) -> SpriteSet {
        SpriteSet { set_name: set_name }
    }

    pub fn list_files(self) -> Result<fs::ReadDir, std::io::Error> {
        let sn = self.set_name;
        fs::read_dir(format!("./sprites/{sn}"))
    }
}
