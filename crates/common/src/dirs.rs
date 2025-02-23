use directories::BaseDirs;
use std::{fs, path::PathBuf};

pub struct Dirs {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl Dirs {
    pub fn new(app_name: impl Into<String>) -> Dirs {
        let home_dir = BaseDirs::new()
            .expect("Could not determine home directory")
            .home_dir()
            .to_path_buf();

        let base = home_dir.join(format!(".{}", app_name.into()));

        let config_dir = base.join("config");
        let data_dir = base.join("data");
        let cache_dir = base.join("cache");

        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");

        Dirs {
            config_dir,
            data_dir,
            cache_dir,
        }
    }
}
