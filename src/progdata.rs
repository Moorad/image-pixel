use std::{fs, path::Path};

use crate::config::Config;

fn get_home_dir() -> String {
    #[cfg(unix)]
    return std::env::var("HOME").expect("No HOME directory");

    #[cfg(windows)]
    return std::env::var("APP_DATA").expect("No APP_DATA directory");
}

fn safe_create_dir(dir: String) -> Result<(), std::io::Error> {
    let path = Path::new(dir.as_str());

    if !path.exists() {
        return fs::create_dir(dir);
    }

    return Ok(());
}

fn safe_delete_dir(dir: String) -> Result<(), std::io::Error> {
    let path = Path::new(dir.as_str());

    if path.exists() {
        return fs::remove_dir_all(dir);
    }

    return Ok(());
}

pub fn init(cfg: &Config) -> Result<(), std::io::Error> {
    safe_create_dir(get_home_dir() + "/.imgpx")?;

    if cfg.zip_sprite_set {
        safe_create_dir(get_home_dir() + "/.imgpx/temp")?;
    }

    return Ok(());
}

pub fn tear_down() -> Result<(), std::io::Error> {
    return safe_delete_dir(get_home_dir() + "/.imgpx/temp");
}

pub mod cache {
    pub fn validate(cache_name: &str) -> bool {
        let binding = get_home_dir() + "/.imgpx/" + cache_name + ".json";
        let path: &Path = Path::new(binding.as_str());
        return path.exists();
    }

    use crate::progdata::get_home_dir;
    use serde::de::DeserializeOwned;
    use serde::ser::Serialize;
    use std::{fs::File, path::Path};

    pub fn save<T>(cache_name: &str, data: T) -> Result<(), serde_json::Error>
    where
        T: Serialize,
    {
        let file = File::create(get_home_dir() + "/.imgpx/" + cache_name + ".json").unwrap();
        serde_json::to_writer(file, &data)
    }

    pub fn load<T>(cache_name: &str) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        let file = File::open(get_home_dir() + "/.imgpx/" + cache_name + ".json").unwrap();
        serde_json::from_reader(file)
    }
}

pub mod zip {
    use std::{
        fs::{self, File},
        io::{self, Error},
    };

    use crate::progdata::get_home_dir;

    pub fn unzip(zip_file: &str) -> Result<String, Error> {
        let fname = std::path::Path::new(zip_file);
        let zip_file = File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let output = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            // fs::create_dir(path)
            let mut outfile = fs::File::create(
                get_home_dir() + "/.imgpx/temp/" + output.file_name().unwrap().to_str().unwrap(),
            )
            .unwrap();

            io::copy(&mut file, &mut outfile)?;
        }

        return Ok(get_home_dir() + "/.imgpx/temp");
    }
}
