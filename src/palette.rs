use std::{collections::BTreeMap, path::PathBuf};

use uuid::Uuid;

use crate::color::Color;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Palette {
    name: String,
    uuid: Uuid,
    colors: BTreeMap<String, Color>,
    filename: Option<PathBuf>,
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

    pub fn color_name(&self) -> Vec<String> {
        self.colors.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }
}

#[cfg(test)]
mod palette_tests {
    use super::*;

    #[test]
    fn test_create_palette() {
        let input = Palette::new("Test Palette");
        assert_eq!(input.name, "Test Palette".to_string());
    }

    #[test]
    fn test_add_color() {
        let mut input = Palette::new("Test Palette");
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
        let mut input = Palette::new("Test Palette");
        let testblue = Color::new_solid(0, 0, 255);
        let _ = input.add_color("Test Blue".to_string(), testblue);
        assert_eq!(input.get_color("Test Blue").unwrap(), &testblue)
    }

    #[test]
    fn test_remove_color() {
        let mut input = Palette::new("Test Palette");
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
        let mut input = Palette::new("Test Palette");
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
        let mut palette = Palette::new("Test Palette");
        let color1 = ("Test Color 1".to_string(), Color::from_hex("#FFF"));
        let color2 = ("Test Color 2".to_string(), Color::from_hex("#F00"));
        assert_eq!(palette.len(), 0);
        let _ = palette.add_color(color1.0, color1.1);
        let _ = palette.add_color(color2.0, color2.1);
    }
}
