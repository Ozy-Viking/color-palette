// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
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
    // config::config();
    let ui = AppWindow::new()?;
    let available_palette: Vec<SharedString> = color_palettes
        .iter()
        .map(|pal| pal.name.clone().into())
        .collect();
    let palette = color_palettes[0].to_slint();
    ui.set_palette(palette);
    ui.set_available_palette(ModelRc::new(VecModel::from(available_palette)));

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });
    // ui.select_palette({
    //     let palette = ui.as_weak();
    //
    // });

    ui.run()
    // todo!();
}
