use crate::printer::tableprint;
use colored::*;
use std::fmt;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Item {
    name: String,
    text: String,
    icon: String,
}

pub struct ItemList {
    items: Vec<Item>,
    mode: String,
    icon: bool,
}

impl Item {
    pub fn new(path: PathBuf, mode: String) -> Item {
        let name: String = path.file_name().unwrap().to_str().unwrap().to_string();
        let is_hide = name.starts_with(".");
        let suffix = if path.is_file() {
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("none_file")
                .to_string()
        } else {
            String::from("dir")
        };
        let (icon, colored_name) = if is_hide {
            if suffix == "dir" {
                ("󱞞", name.dimmed())
            } else {
                ("󰘓", name.dimmed())
            }
        } else {
            match suffix.as_str() {
                "dir" => ("", name.blue()),
                "rs" => ("", name.green()),
                "md" => ("", name.white()),
                "toml" => ("", name.yellow()),
                "lock" => ("󰈡", name.red()),
                "json" => ("", name.bright_yellow()),
                "txt" => ("󰈙", name.cyan()),
                "html" => ("", name.red()),
                "css" => ("", name.blue()),
                "js" => ("", name.yellow()),
                "py" => ("", name.blue()),
                "exe" => ("", name.red()),
                "bat" => ("", name.red()),
                "sh" => ("", name.green()),
                _ => ("󰈔", name.white()),
            }
        };

        let print = match mode.as_str() {
            "-color" | "-ln" => Item {
                name,
                text: colored_name.to_string(),
                icon: "".to_string(),
            },
            "-nocolor" | "-nocolorln" => Item {
                name: name.clone(),
                text: name.to_string(),
                icon: "".to_string(),
            },
            "-icon" | "-iconln" => Item {
                name: name.clone(),
                text: colored_name.to_string(),
                icon: icon.to_string(),
            },
            _ => Item {
                name: name.clone(),
                text: name.to_string(),
                icon: "".to_string(),
            },
        };

        return print;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_icon(&self) -> &String {
        &self.icon
    }

}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 如果有图标，将图标和文本一起显示
        if !self.icon.is_empty() {
            write!(f, "{} {}", self.icon, self.text)
        } else {
            // 否则只显示文本
            write!(f, "{}", self.text)
        }
    }
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item {{ name: {}, text: {}, icon: {}}}", self.name, self.text, self.icon)
    }
}

impl ItemList {
    pub fn new(mode: String) -> ItemList {
        ItemList {
            items: vec![],
            mode: { mode.clone() },
            icon: { mode.starts_with("-icon") },
        }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn is_icon(&self) -> bool {
        self.icon
    }

    pub fn print(&self) {
        if self.mode.ends_with("ln") {
            if self.icon {
                for item in &self.items {
                    println!("{} {}", item.icon, item.text);
                }
            } else {
                for item in &self.items {
                    println!("{}", item.text);
                }
            }
        } else {
            tableprint(self);
        }
    }
}

impl<'a> IntoIterator for &'a ItemList {
    type Item = &'a Item;
    type IntoIter = std::slice::Iter<'a, Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

