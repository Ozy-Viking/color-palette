use std::{collections::BTreeMap, path::PathBuf};

use slint::ModelRc;
use uuid::Uuid;

use super::{ColorType, PaletteType, Theme};
use crate::color::Color;
use slint::Color as Slint_Color;
use slint::VecModel;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Palette {
    pub name: String,
    pub uuid: Uuid,
    pub colors: BTreeMap<String, Color>,
    pub filename: Option<PathBuf>,
}

pub struct ColorScheme {
    pub foreground: Slint_Color,
    pub background: Slint_Color,
}

#[allow(dead_code)]
impl Palette {
    pub fn new(name: &str, filename: Option<PathBuf>) -> Self {
        let uuid = Uuid::now_v6(&[1, 2, 3, 4, 5, 6]);
        Palette {
            name: name.to_string(),
            uuid,
            colors: BTreeMap::new(),
            filename,
        }
    }

    pub fn add_color(&mut self, name: String, color: Color) -> Option<Color> {
        self.colors.insert(name, color)
    }

    pub fn remove_color(&mut self, name: &str) -> Option<(String, Color)> {
        self.colors.remove_entry(name)
    }

    pub fn get_color(&self, name: &str) -> Option<&Color> {
        self.colors.get(name)
    }

    pub fn color_names(&self) -> Vec<String> {
        self.colors.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn to_slint(&self) -> PaletteType {
        PaletteType {
            colors: ModelRc::new(VecModel::from(
                self.colors
                    .iter()
                    .map(|(key, value)| value.to_colortype(key.as_str()))
                    .collect::<Vec<ColorType>>(),
            )),
            name: self.name.clone().into(),
        }
    }

    pub fn background(&self) -> Option<Color> {
        let color_names = self.color_names();
        let bg_key_option: Option<&String> = color_names
            .iter()
            .find(|name| name.to_lowercase() == "bg" || name.to_lowercase() == "background");
        let bg_key = match bg_key_option {
            None => return None,
            Some(key) => key.to_owned(),
        };

        let color = Some(self.get_color(&bg_key).unwrap().to_owned());
        color
    }

    pub fn foreground(&self) -> Option<Color> {
        let color_names = self.color_names();
        let fg_key_option: Option<&String> = color_names.iter().find(|name| {
            name.to_lowercase().trim() == "fg"
                || name.to_lowercase().trim() == "foreground"
                || name.to_lowercase().trim() == "text"
        });
        let fg_key = match fg_key_option {
            None => return None,
            Some(key) => key.to_owned(),
        };

        let color = Some(self.get_color(&fg_key).unwrap().to_owned());
        color
    }

    pub fn to_slint_theme(&self) -> Option<ColorScheme> {
        let background = match self.background() {
            None => return None,
            Some(bg) => bg.to_opaque().to_slint(),
        };
        let foreground = match self.foreground() {
            None => return None,
            Some(fg) => fg.to_opaque().to_slint(),
        };
        Some(ColorScheme {
            foreground,
            background,
        })
    }
}

#[cfg(test)]
mod palette_tests {
    use super::*;

    #[test]
    fn test_create_palette() {
        let input = Palette::new("Test Palette", None);
        assert_eq!(input.name, "Test Palette".to_string());
    }

    #[test]
    fn test_add_color() {
        let mut input = Palette::new("Test Palette", None);
        let testblue1 = Color::new_solid(0, 0, 255);
        let testblue2 = Color::new_solid(0, 0, 255);
        let first = input.add_color("Test Blue".to_string(), testblue1);
        assert_eq!(*input.colors.get("Test Blue").unwrap(), testblue1);
        assert_eq!(first, None);
        assert_eq!(
            input.add_color("Test Blue".to_string(), testblue2),
            Some(testblue1)
        )
    }

    #[test]
    fn test_get_color() {
        let mut input = Palette::new("Test Palette", None);
        let testblue = Color::new_solid(0, 0, 255);
        let _ = input.add_color("Test Blue".to_string(), testblue);
        assert_eq!(input.get_color("Test Blue").unwrap(), &testblue)
    }

    #[test]
    fn test_remove_color() {
        let mut input = Palette::new("Test Palette", None);
        let testblue = Color::new_solid(0, 0, 255);
        let _ = input.add_color("Test Blue".to_string(), testblue);
        assert_eq!(*input.colors.get("Test Blue").unwrap(), testblue);
        assert_eq!(input.colors.len(), 1);
        assert_eq!(
            input.remove_color("Test Blue"),
            Some(("Test Blue".to_string(), testblue))
        );
        assert_eq!(input.colors.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut input = Palette::new("Test Palette", None);
        assert_eq!(input.len(), 0);
        let testblue = Color::new_solid(0, 0, 255);
        let _ = input.add_color("Test Blue".to_string(), testblue);
        assert_eq!(input.len(), 1);
        let testblue = Color::new_solid(0, 0, 255);
        let _ = input.add_color("Test Blue 2".to_string(), testblue);
        assert_eq!(input.len(), 2);
        let testblue = Color::new_solid(0, 0, 255);
        let _ = input.add_color("Test Blue 2".to_string(), testblue);
        assert_eq!(input.len(), 2);
    }

    #[test]
    fn color_names() {
        let mut palette = Palette::new("Test Palette", None);
        let color1 = ("Test Color 1".to_string(), Color::from_hex("#FFF"));
        let color2 = ("Test Color 2".to_string(), Color::from_hex("#F00"));
        assert_eq!(palette.len(), 0);
        let _ = palette.add_color(color1.0, color1.1);
        let _ = palette.add_color(color2.0, color2.1);
    }
}
