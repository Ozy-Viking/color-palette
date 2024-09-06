use config::{Config, File, FileFormat};
use std::path::PathBuf;

pub const DEFAULT_SETTINGS: &str = "\
[General]
test = false
Background = \"#000\"
font = \"CaskaydiaCove 24pt\"
foreground = \"#FFF\"
";

pub fn read_default_config_file(file_path: PathBuf) -> Option<Config> {
    let settings_builder = Config::builder()
        .add_source(File::from_str(DEFAULT_SETTINGS, FileFormat::Yaml))
        .add_source(config::File::with_name(file_path.to_str()?));

    match settings_builder.build() {
        Ok(settings) => Some(settings),
        Err(_) => None,
    }
}
