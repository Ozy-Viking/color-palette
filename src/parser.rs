// TODO: Css, toml, json, config
use toml::Table;

#[allow(dead_code)]
enum ParserType {
    Toml,
}

#[allow(dead_code)]
fn toml_titles(input: String) -> Vec<String> {
    let toml = input.parse::<Table>().unwrap();
    toml.keys().cloned().collect::<Vec<String>>()
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_toml_title() {
        let input: String = read_to_string("test/test.toml").expect("File in place.");
        assert_eq!(toml_titles(input), vec!["Color Scheme Title"]);
    }
}

// {"Color Scheme Title": Table({"background": String("#0000017D"), "foreground": String("#FFFFFF"), "color0": String("#110f1e"), "color1": String("#C14039"), "color2": String("#076D8E"), "color3": String("#5A6592"), "color4": String("#0794B1"), "color5": String("#5BA5B8"), "color6": String("#03B1CC"), "color7": String("#78ddea"), "color8": String("#549aa3"), "color9": String("#C14039"), "color10": String("#076D8E"), "color11": String("#5A6592"), "color12": String("#0794B1"), "color13": String("#5BA5B8"), "color14": String("#03B1CC"), "color15": String("#78ddea")})}
