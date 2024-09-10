use config::{Config, File, FileFormat, Map};
use glob::glob;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use crate::color::Color;
use crate::palette::Palette;

pub const DEFAULT_SETTINGS: &str = "
[General]
test = false
font = \"CaskaydiaCove NF\"
fontsize = \"14px\"
foreground = \"#FFF\"
background = \"#000\"
";

pub fn read_program_config_file(file_path: &Path) -> Option<Config> {
    let settings_builder = Config::builder()
        .add_source(File::from_str(DEFAULT_SETTINGS, FileFormat::Toml))
        .add_source(config::File::with_name(file_path.to_str()?));

    match settings_builder.build() {
        Ok(settings) => Some(settings),
        Err(_) => None,
    }
}

pub fn read_colour_palettes(config_folder: &Path) -> Option<Vec<Palette>> {
    let palette_folder = config_folder.join("palettes/").to_str().unwrap().to_owned();
    println!("{:?}", &palette_folder);
    let pattern = palette_folder + "*.toml";

    let files = glob(pattern.as_str())
        .unwrap()
        .map(|path| File::from(path.unwrap()))
        .collect::<Vec<_>>();

    println!("{:?}", &files);

    let settings_builder = Config::builder().add_source(files);

    let palette_config = match settings_builder.build() {
        Ok(settings) => settings,
        Err(_) => {
            return None;
        }
    };

    let palettes_map = palette_config
        .clone()
        .try_deserialize::<Map<String, Map<String, String>>>()
        .unwrap();

    if palettes_map.is_empty() {
        return None;
    }

    let mut palettes: Vec<Palette> = Vec::new();
    let palette_names = palettes_map.keys().cloned();
    for palette in palette_names {
        let mut pal = Palette::new(palette.as_str(), None);
        let filename = palette_config.clone();
        println!("{:?}", filename);
        for (name, color) in &palettes_map[&palette] {
            pal.add_color(name.to_owned(), Color::from_hex(color));
        }
        palettes.push(pal);
    }
    Some(palettes)
}

pub fn get_pallete_filesnames(palette_folder: PathBuf) -> Result<Vec<PathBuf>, String> {
    if !palette_folder.is_dir() {
        return Err("palettes_folder is not a directory".to_string());
    }
    let mut palette_files: Vec<PathBuf> = Vec::new();
    for file in fs::read_dir(palette_folder).expect("Palette folder created already.") {
        palette_files.push(file.expect("If it fails, run.").path());
    }
    Ok(palette_files)
}
