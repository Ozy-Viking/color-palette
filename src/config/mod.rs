use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

use reader::get_pallete_filesnames;
use slint::format;

mod reader;
mod writer;

pub const DEFAULT_CONFIG_PATH: &str = "~/.config";
pub const PROGRAM_NAME: &str = "color-palette";

struct Config {
    current_settings: BTreeMap<String, String>,
    default_settings: BTreeMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            current_settings: BTreeMap::new(),
            default_settings: BTreeMap::from([(
                "program name".to_string(),
                PROGRAM_NAME.to_string(),
            )]),
        }
    }
}

pub fn config() {
    let conf_path = find_config_path().unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));
    let config_folder = ensure_config_folder_exists(&conf_path).expect("Unable to create a config folder, please set 'XDG_CONFIG_HOME' in environment variables before running again.");
    let program_config_file = ensure_program_config_file_exists(&config_folder).expect("");
    let palette_folder =
        ensure_palette_folder_exists(&config_folder).expect("Folder should be good");
    // reader::read_program_config_file(program_config_file);
    let palette_filenames = get_pallete_filesnames(palette_folder).unwrap();
    let settings = reader::read_program_config_file(&program_config_file).unwrap();
    let palettes = reader::read_colour_palettes(&config_folder);

    // println!("{:?}", &palettes);
}

fn find_config_path() -> Option<PathBuf> {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(config_home));
    }
    None
}

fn ensure_config_folder_exists(config_folder_path: &Path) -> Result<PathBuf, String> {
    let program_config_folder = config_folder_path.join(PROGRAM_NAME);
    for _ in 0..2 {
        match fs::metadata(&program_config_folder) {
            Ok(folder_meta) => {
                if folder_meta.is_dir() {
                    return Ok(program_config_folder);
                }
                return Err(format!("There is a file with the name: {}", PROGRAM_NAME).to_string());
            }
            Err(_) => {
                fs::create_dir(&program_config_folder)
                    .expect("User should have perms for config folder");
            }
        };
    }
    Err("Unable to create a config folder, please set 'XDG_CONFIG_HOME' in environment variables before running again.".to_string())
}

fn ensure_program_config_file_exists(program_config_folder: &Path) -> Result<PathBuf, String> {
    let file_name = PROGRAM_NAME.to_owned() + ".toml";
    let general_config_file = program_config_folder.join(&file_name);
    for _ in 0..2 {
        match fs::metadata(&general_config_file) {
            Ok(file_meta) => {
                if file_meta.is_file() {
                    return Ok(general_config_file);
                }
                return Err(format!("There is a folder called: {}", &file_name).to_string());
            }
            Err(_) => {
                fs::write(&general_config_file, b"\n")
                    .expect("User should have perms for config folder");
            }
        };
    }

    Err("User should have perms for config folder".to_string())
}

fn ensure_palette_folder_exists(config_folder_path: &Path) -> Result<PathBuf, String> {
    let palette_folder = config_folder_path.join("palettes");
    for _ in 0..2 {
        match fs::metadata(&palette_folder) {
            Ok(folder_meta) => {
                if folder_meta.is_dir() {
                    return Ok(palette_folder);
                }
                return Err(format!("There is a file with the name: {}", PROGRAM_NAME).to_string());
            }
            Err(_) => {
                fs::create_dir(&palette_folder).expect("User should have perms for config folder");
            }
        };
    }

    Err("Unable to create palette folder in the config folder.".to_string())
}
