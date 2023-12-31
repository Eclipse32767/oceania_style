#![allow(dead_code)]

use std::{env, fs};
use iced_style::{button, pick_list, menu, theme::Palette};
use iced::{Color, Background};
use iced::theme::{self, Theme};
use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str};
use std::fs::read_to_string;

pub fn get_home() -> String {
    match env::var("XDG_CONFIG_HOME") {
        Ok(var) => var,
        Err(..) => match env::var("HOME") {
            Ok(var) => format!("{var}/.config"),
            Err(..) => panic!("Failed to find config directory, make sure XDG_CONFIG_HOME or HOME are set")
        }
    }
}

#[derive(Clone)]
pub struct ButtonStyle {
    pub border_radius: f32,
    pub txt_color: Color,
    pub bg_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub shadow_offset: iced::Vector
}
impl button::StyleSheet for ButtonStyle {
    type Style = Theme;
    fn active(&self, _style: &Theme) -> button::Appearance {
        button::Appearance {
            shadow_offset: self.shadow_offset.clone(),
            border_radius: self.border_radius.clone().into(),
            text_color: self.txt_color.clone(),
            border_color: self.border_color.clone(),
            border_width: self.border_width.clone(),
            background: Some(Background::Color(self.bg_color.clone())),
        }
    }
}
pub struct TextStyle {
    pub color: Color
}
impl TextStyle {
    pub fn mk_theme(&self) -> theme::Text {
        theme::Text::Color(self.color.clone())
    }
}
impl ButtonStyle {
    pub fn mk_theme(&self) -> theme::Button {
        theme::Button::Custom(Box::new(self.clone()))
    }
}
pub fn mk_app_theme(palette: Palette) -> Theme {
    Theme::Custom(Box::new(theme::Custom::new(palette)))
}

#[derive(Clone)]
pub struct ListStyle {
    pub txt_color: Color,
    pub bg_color: Color,
    pub handle_color: Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
    pub menu: MenuStyle,
}
impl pick_list::StyleSheet for ListStyle {
    type Style = Theme;
    fn active(&self, _style: &Theme) -> pick_list::Appearance {
        pick_list::Appearance { 
            text_color: self.txt_color.clone(), 
            placeholder_color: Color::from_rgb8(0xFF, 0x00, 0x00), 
            handle_color: self.handle_color.clone(), 
            background: Background::Color(self.bg_color.clone()),
            border_radius: self.border_radius.into(), 
            border_width: self.border_width, 
            border_color: self.border_color.clone() 
        }
    }
    fn hovered(&self, _style: &Theme) -> pick_list::Appearance {
        pick_list::Appearance { 
            text_color: self.txt_color.clone(), 
            placeholder_color: Color::from_rgb8(0xFF, 0x00, 0x00), 
            handle_color: self.handle_color.clone(), 
            background: Background::Color(self.bg_color.clone()),
            border_radius: self.border_radius.into(), 
            border_width: self.border_width, 
            border_color: self.border_color.clone() 
        }
    }
}
#[derive(Clone)]
pub struct MenuStyle {
    pub txt_color: Color,
    pub bg_color: Color,
    pub border_width: f32,
    pub border_radius: f32,
    pub border_color: Color,
    pub sel_txt_color: Color,
    pub sel_bg_color: Color
}
pub struct ButtonPair {
    pub active: ButtonStyle,
    pub inactive: ButtonStyle
}
#[derive(Clone)]
pub struct ThemeCustom {
    pub application: Palette,
    pub secondary: ButtonStyle,
    pub sidebar: ButtonStyle,
    pub list: ListStyle,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct ThemeFile {
    pub bg_color1: String,
    pub bg_color2: String,
    pub bg_color3: String,
    pub txt_color: String,
    pub red: String,
    pub orange: String,
    pub yellow: String,
    pub green: String,
    pub blue: String,
    pub purple: String,
    pub pink: String
}

pub fn string_to_color(x: String) -> Color {
    let mut split_str = x.split_at(2);
    let red_str = split_str.0;
    split_str = split_str.1.split_at(2);
    let green_str = split_str.0;
    let blue_str = split_str.1;
    let red_num = u8::from_str_radix(red_str, 16).expect("failed to parse red value");
    let green_num = u8::from_str_radix(green_str, 16).expect("failed to parse green value");
    let blue_num = u8::from_str_radix(blue_str, 16).expect("failed to parse blue value");

    Color::from_rgb8(red_num, green_num, blue_num)
}
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
pub fn string_from_col(color: &Color) -> String {
    let rgba = color.into_rgba8();
    let prepend_0 = [rgba[0] < 16, rgba[1] < 16, rgba[2] < 16];
    let red_str = format_radix(rgba[0].into(), 16);
    let green_str = format_radix(rgba[1].into(), 16);
    let blue_str = format_radix(rgba[2].into(), 16);
    let mut output = match prepend_0[0] {
        true => format!("0{red_str}"),
        false => format!("{red_str}")
    };
    output = match prepend_0[1] {
        true => format!("{output}0{green_str}"),
        false => format!("{output}{green_str}"),
    };
    match prepend_0[2] {
        true => format!("{output}0{blue_str}"),
        false => format!("{output}{blue_str}")
    }
}
pub fn make_custom_theme() -> ThemeCustom {
    let home = get_home();
    let path = format!("{home}/Oceania/theme.toml");
    let backup_path = format!("{home}/Oceania");
    let placeholder = r#"bg_color1 = "181926"
    bg_color2 = "1e2030"
    bg_color3 = "24273a"
    txt_color = "cad3f5"
    red = "ed8796"
    orange = "f5a97f"
    yellow = "eed49f"
    green = "a6da95"
    blue = "8aadf4"
    purple = "c6a0f6"
    pink = "f5bde6""#;
    let file = match read_to_string(path.clone()) {
        Ok(var) => var,
        Err(..) => match read_to_string("/etc/Oceania/theme.toml") {
            Ok(var) => var,
            Err(..) => {
                std::process::Command::new("mkdir").arg("-p").arg(backup_path).output().expect("uh oh");
                fs::write(path, placeholder.clone()).expect("failed to write backup file");
                placeholder.to_string()
            }
        }
    };
    let decoded: ThemeFile = from_str(&file).unwrap();

    ThemeCustom {
        application: Palette {
            background: string_to_color(decoded.bg_color1.clone()),
            text: string_to_color(decoded.txt_color.clone()),
            primary: string_to_color(decoded.blue.clone()),
            success: string_to_color(decoded.green.clone()),
            danger: string_to_color(decoded.red.clone())
        },
        secondary: ButtonStyle {
            border_radius: 2.0,
            txt_color: string_to_color(decoded.txt_color.clone()),
            bg_color: string_to_color(decoded.bg_color3.clone()),
            border_color: Color::from_rgb8(0, 0, 0),
            border_width: 0.0,
            shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
        },
        sidebar: ButtonStyle {
            border_radius: 2.0,
            txt_color: string_to_color(decoded.txt_color.clone()),
            bg_color: string_to_color(decoded.bg_color2.clone()),
            border_color: Color::from_rgb8(0, 0, 0),
            border_width: 0.0,
            shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
        },
        list: ListStyle {
            txt_color: string_to_color(decoded.txt_color.clone()),
            bg_color: string_to_color(decoded.bg_color3.clone()),
            handle_color: string_to_color(decoded.txt_color.clone()),
            border_radius: 5.0,
            border_width: 2.0,
            border_color: string_to_color(decoded.txt_color.clone()),
            menu: MenuStyle {
                txt_color: string_to_color(decoded.txt_color.clone()),
                bg_color: string_to_color(decoded.bg_color3.clone()),
                border_width: 2.0,
                border_radius: 5.0,
                border_color: string_to_color(decoded.txt_color.clone()),
                sel_txt_color: string_to_color(decoded.txt_color.clone()),
                sel_bg_color: string_to_color(decoded.blue.clone())
            },
        },
    }
}

impl menu::StyleSheet for MenuStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Theme) -> menu::Appearance {
        menu::Appearance { 
            text_color: self.txt_color.clone(), 
            background: Background::Color(self.bg_color.clone()), 
            border_width: self.border_width, 
            border_radius: self.border_radius.into(), 
            border_color: self.border_color.clone(), 
            selected_text_color: self.sel_txt_color.clone(), 
            selected_background: Background::Color(self.sel_bg_color.clone())
        }
    }
}
impl ListStyle {
    pub fn mk_theme(&self) -> theme::PickList {
        theme::PickList::Custom(std::rc::Rc::new(self.clone()),std::rc::Rc::new(self.menu.clone()))
    }
}
pub struct ThemeSet {
    pub light: ThemeCustom,
    pub dark: ThemeCustom,
    pub custom: ThemeCustom
}
#[derive(Debug, Clone)]
pub enum SelectedTheme {
    Light,
    Dark,
    Custom
}
#[derive(Serialize, Deserialize)]
pub struct CuttlefishCfg {//struct used in collecting the user's preferred theme
pub theme: String
}
pub fn get_set_theme() -> SelectedTheme {//function to retrieve the user's theme preference
    let home = format!("{}/Oceania/cfg.toml", get_home());
    match read_to_string(home) {
        Ok(x) => {
            let cfg: CuttlefishCfg = from_str(&x).unwrap();
            let theme_str = cfg.theme.clone();
            if theme_str == String::from("dark") {
                SelectedTheme::Dark
            } else if theme_str == String::from("custom") {
                SelectedTheme::Custom
            } else {
                SelectedTheme::Light
            }
        }
        Err(..) => SelectedTheme::Light
    }
}
