use std::{fs, path::Path};

fn get_home_dir() -> String {
    #[cfg(unix)]
    return std::env::var("HOME").expect("No HOME directory");

    #[cfg(windows)]
    return std::env::var("APP_DATA").expect("No APP_DATA directory");
}

pub fn init() -> Result<(), std::io::Error> {
    let binding = get_home_dir() + "/.imgpx";
    let path = Path::new(binding.as_str());
    if !path.exists() {
        return fs::create_dir(get_home_dir() + "/.imgpx");
    }

    return Ok(());
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
