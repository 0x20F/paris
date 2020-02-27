use colored::Color;
use std::fmt::{Display, Error};



pub struct Colors {}


impl Colors {
    pub fn get(key: &str) -> String {
        let reset_key = "/";

        if key == reset_key {
            return String::from("\x1B[0m");
        }

        let color: Color = key.split_whitespace().last().unwrap().into();
        let mut res = String::from("\x1B[");

        if key.starts_with("bg") {
            res.push_str(color.to_bg_str());
        } else {
            res.push_str(color.to_fg_str());
        }

        res.push('m');
        res
    }
}



// Use for adding bold, underline, background colors, foreground colors
pub struct Formatter {}


impl Formatter {

}