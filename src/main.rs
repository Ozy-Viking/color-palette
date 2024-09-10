#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::ComponentHandle;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;

mod color;
mod config;
mod palette;
mod parser;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let config_folder = config::config_palette_folder();
    let color_palettes = config::reader::read_colour_palettes(&config_folder).unwrap();

    let ui = AppWindow::new()?;
    let available_palette: Vec<SharedString> = color_palettes
        .iter()
        .map(|pal| pal.name.clone().into())
        .collect();
    let palette = &color_palettes[0];
    ui.set_palette(palette.to_slint());
    if let Some(theme) = palette.to_slint_theme() {
        ui.global::<Theme>().set_background(theme.background);
        ui.global::<Theme>().set_foreground(theme.foreground);
    }
    ui.set_available_palette(ModelRc::new(VecModel::from(available_palette)));
    ui.global::<Copy>().on_copy_on_click(move |text| {
        println!("{:?}", text);
        cli_clipboard::set_contents(text.to_string()).expect("");
    });

    ui.run()
}
