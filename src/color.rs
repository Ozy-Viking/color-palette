use slint::format;
use std::ops::Rem;
use winnow::stream::SliceLen;

pub const PRECISION_VAL: u32 = 5;

/// # Color Struct
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    /// red: 0-255
    pub red: u8,
    /// green: 0-255
    pub green: u8,
    /// blue: 0-255
    pub blue: u8,
    /// opacity: 0-255
    pub opacity: u8,
}

#[allow(dead_code)]
impl Color {
    /// If you are using solid colors, use [`Self::new_solid`].
    pub fn new(red: u8, green: u8, blue: u8, opacity: u8) -> Self {
        Color {
            red,
            green,
            blue,
            opacity,
        }
    }
    pub fn new_solid(red: u8, green: u8, blue: u8) -> Self {
        Color {
            red,
            green,
            blue,
            opacity: 255,
        }
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    pub fn str_rgb(&self) -> String {
        format!("{}, {}, {}", self.red, self.blue, self.blue).to_string()
    }

    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.opacity)
    }

    pub fn str_rgba(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.red, self.blue, self.blue, self.opacity
        )
        .to_string()
    }

    pub fn hsl(&self) -> (f64, f64, f64) {
        let r_prime = self.red as f64 / 255_f64;
        let g_prime = self.green as f64 / 255_f64;
        let b_prime = self.blue as f64 / 255_f64;

        let cmax = r_prime.max(g_prime.max(b_prime));
        let cmin = r_prime.min(g_prime.min(b_prime));

        let delta = cmax - cmin;
        let mut hue: f64;
        if delta == 0.0 {
            hue = 0.0;
        } else if cmax == r_prime {
            hue = 60.0 * (((g_prime - b_prime) / delta).rem(6.0));
        } else if cmax == g_prime {
            hue = 60.0 * (((b_prime - r_prime) / delta) + 2.0);
        } else if cmax == b_prime {
            hue = 60.0 * (((r_prime - g_prime) / delta) + 4.0);
        } else {
            panic!("Should of matched.")
        }
        if hue < 0.0 {
            hue += 360.0;
        }

        let mut lightness = (cmax + cmin) / 2.0;

        let mut sat = match delta {
            0.0 => 0.0,
            _ => delta / (1.0 - (2.0 * lightness - 1.0).abs()),
        };
        hue = round_to(hue, PRECISION_VAL);
        sat = round_to(sat, PRECISION_VAL);
        lightness = round_to(lightness, PRECISION_VAL);
        (hue, sat, lightness)
    }

    pub fn hsla(&self) -> (f64, f64, f64, f64) {
        let (h, s, l) = self.hsl();
        let a = round_to(self.opacity as f64 / 255_f64, PRECISION_VAL);
        (h, s, l, a)
    }

    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue).to_string()
    }

    pub fn hexa(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.opacity
        )
        .to_string()
    }

    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            opacity: alpha,
        }
    }
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Color {
            red,
            green,
            blue,
            opacity: 255,
        }
    }

    pub fn from_hex(hex: &str) -> Self {
        let hex_color: String = hex.trim_start_matches("#").to_uppercase();

        let colour = match hex_color.len() {
            3 => {
                let red = u8::from_str_radix(double_letter(&hex_color.as_str()[0..1]).as_str(), 16)
                    .unwrap();
                let green =
                    u8::from_str_radix(double_letter(&hex_color.as_str()[1..2]).as_str(), 16)
                        .unwrap();
                let blue = u8::from_str_radix(double_letter(&hex_color.as_str()[2..]).as_str(), 16)
                    .unwrap();

                Color::new_solid(red, green, blue)
            }
            6 => {
                let red = u8::from_str_radix(&hex_color[0..2], 16).unwrap();
                let green = u8::from_str_radix(&hex_color[2..4], 16).unwrap();
                let blue = u8::from_str_radix(&hex_color[4..6], 16).unwrap();
                Color::new_solid(red, green, blue)
            }
            8 => {
                let red = u8::from_str_radix(&hex_color[0..2], 16).unwrap();
                let green = u8::from_str_radix(&hex_color[2..4], 16).unwrap();
                let blue = u8::from_str_radix(&hex_color[4..6], 16).unwrap();
                let opacity = u8::from_str_radix(&hex_color[6..8], 16).unwrap();
                Color::new(red, green, blue, opacity)
            }
            _ => {
                panic!("Wrong Size Hex.");
            }
        };
        colour
    }

    pub fn from_hsl() -> Self {
        todo!();
    }

    pub fn from_hsla() -> Self {
        todo!();
    }
}

fn double_letter(letter: &str) -> String {
    if letter.slice_len() != 1 {
        return letter.to_string();
    }
    format!("{}{}", letter, letter).to_string()
}

fn round_to(number: f64, n: u32) -> f64 {
    let precision: f64 = 10_u32.pow(n) as f64;
    (number * precision).round() / precision
}

#[cfg(test)]
mod test_colour {
    use super::*;

    #[test]
    fn new_color_with_opacity() {
        let input_color = Color::new(255, 255, 255, 255);
        assert_eq!(input_color.red, 255);
        assert_eq!(input_color.green, 255);
        assert_eq!(input_color.blue, 255);
        assert_eq!(input_color.opacity, 255);
    }

    #[test]
    fn new_color_without_opacity() {
        let input_color = Color::new_solid(255, 255, 255);
        assert_eq!(input_color.red, 255);
        assert_eq!(input_color.green, 255);
        assert_eq!(input_color.blue, 255);
        assert_eq!(input_color.opacity, 255);
    }

    #[test]
    fn rgb() {
        let input_color = Color::new(125, 125, 233, 198);
        assert_eq!(input_color.rgb(), (125, 125, 233));
    }

    #[test]
    fn rgba() {
        let input_color = Color::new(125, 125, 233, 198);
        assert_eq!(input_color.rgba(), (125, 125, 233, 198));
    }

    #[test]
    fn hsl() {
        assert_eq!(Color::new_solid(0, 0, 0).hsl(), (0.0, 0.0, 0.0));
        assert_eq!(Color::new_solid(255, 255, 255).hsl(), (0.0, 0.0, 1.0));
        assert_eq!(Color::new_solid(255, 0, 0).hsl(), (0.0, 1.0, 0.5));
        assert_eq!(Color::new_solid(0, 255, 0).hsl(), (120.0, 1.0, 0.5));
        assert_eq!(Color::new_solid(0, 0, 255).hsl(), (240.0, 1.0, 0.5));
        assert_eq!(Color::new_solid(255, 0, 255).hsl(), (300.0, 1.0, 0.5));
        assert_eq!(Color::new_solid(191, 191, 191).hsl(), (0.0, 0.0, 0.74902));
        assert_eq!(Color::new_solid(0, 128, 0).hsl(), (120.0, 1.0, 0.25098));
    }
    #[test]
    fn hsla() {
        assert_eq!(Color::new(0, 0, 0, 0).hsla(), (0.0, 0.0, 0.0, 0.0));
        assert_eq!(Color::new(255, 255, 255, 255).hsla(), (0.0, 0.0, 1.0, 1.0));
        assert_eq!(Color::new(255, 0, 0, 125).hsla(), (0.0, 1.0, 0.5, 0.4902));
    }

    #[test]
    fn hex() {
        assert_eq!(Color::new_solid(0, 0, 0).hex(), "#000000");
        assert_eq!(Color::new_solid(255, 255, 255).hex(), "#FFFFFF");
    }

    #[test]
    fn hexa() {
        assert_eq!(Color::new(0, 0, 0, 0).hexa(), "#00000000");
        assert_eq!(Color::new(255, 255, 255, 255).hexa(), "#FFFFFFFF");
    }

    #[test]
    fn from_hex() {
        assert_eq!(Color::from_hex("#000000").rgb(), (0, 0, 0));
        assert_eq!(Color::from_hex("#000").rgb(), (0, 0, 0));
        assert_eq!(Color::from_hex("#00000000").rgba(), (0, 0, 0, 0));
        assert_eq!(Color::from_hex("#FFFFFF").rgb(), (255, 255, 255));
        assert_eq!(Color::from_hex("#FFF").rgb(), (255, 255, 255));
        assert_eq!(Color::from_hex("#FFFFFFFF").rgba(), (255, 255, 255, 255));
        assert_eq!(Color::from_hex("#7F1F00").rgb(), (127, 31, 0));
    }
}
