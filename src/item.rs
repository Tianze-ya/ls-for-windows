use crate::tableprint::tableprint;
use colored::*;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Item {
    name: String,
    print: String,
}

pub struct ItemList {
    items: Vec<Item>,
    mode: String,
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
                ("󰘓", name.dimmed())
            } else {
                ("󱞞", name.dimmed())
            }
        } else {
            match suffix.as_str() {
                "dir" => ("  ", name.blue()),
                "rs" => ("  ", name.green()),
                "md" => ("  ", name.normal()),
                "toml" => ("  ", name.yellow()),
                "lock" => ("󰈡 ", name.red()),
                "gitignore" => ("󰊢 ", name.magenta()),
                "json" => (" ", name.bright_yellow()),
                "txt" => ("󰈙", name.cyan()),
                "html" => (" ", name.red()),
                "css" => (" ", name.blue()),
                "js" => (" ", name.yellow()),
                "py" => (" ", name.blue()),
                "exe" => (" ", name.red()),
                "bat" => (" ", name.yellow()),
                "sh" => (" ", name.green()),
                _ => ("󰈔 ", name.normal()),
            }
        };

        let print = match mode.as_str() {
            "-color" => format!("{}", colored_name),
            "-ln" => format!("{}", colored_name),
            "-nocolor" => format!("{}", name),
            "-nocolorln" => format!("{}", name),
            "-icon" => format!("{}{}", icon, colored_name),
            "-iconln" => format!("{}{}", icon, colored_name),
            _ => format!("{}", name),
        };

        Item { name, print }
    }
}

impl ItemList {
    pub fn new(mode: String) -> ItemList {
        ItemList {
            items: vec![],
            mode,
        }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn print(&self) {
        if &self.mode == "-ln" || &self.mode == "-nocolorln" || &self.mode == "-iconln" {
            for item in &self.items {
                println!("{}", item.print);
            }
        } else {
            tableprint(self.items.iter().map(|i| i.print.clone()).collect());
        }
    }
}
