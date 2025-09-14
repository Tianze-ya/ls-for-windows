use colored::*;
use std::process::Command;
use std::{env, fs, path::PathBuf};

struct Item {
    name: String,
    colored_name: ColoredString,
    icon: &'static str,
}

impl Item {
    fn new(path: PathBuf) -> Item {
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
                "dir" => (" ", name.blue()),
                "rs" => (" ", name.green()),
                "md" => (" ", name.normal()),
                "toml" => (" ", name.yellow()),
                "lock" => ("󰈡", name.red()),
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
        Item {
            name,
            colored_name,
            icon,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        print!("{}  ", self.colored_name);
    }

    #[allow(dead_code)]
    fn println(&self) {
        println!("{}  ", self.colored_name);
    }

    #[allow(dead_code)]
    fn icon_print(&self) {
        print!("{}{}  ", self.icon, self.colored_name);
    }

    #[allow(dead_code)]
    fn icon_println(&self) {
        println!("{}{}  ", self.icon, self.colored_name);
    }

    #[allow(dead_code)]
    fn nocolor_print(&self) {
        print!("{}  ", self.name);
    }

    #[allow(dead_code)]
    fn nocolor_println(&self) {
        println!("{}  ", self.name);
    }
}

fn read_directory(path: &str) -> std::io::Result<Vec<Item>> {
    let entries = fs::read_dir(path)?;
    let mut item: Vec<Item> = vec![];

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        item.push(Item::new(path));
    }

    return Ok(item);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (path, mode) = match args.len() {
        1 => ("./".to_string(), "-color".to_string()),
        2 => {
            if args[1].starts_with("-") {
                ("./".to_string(), args[1].clone())
            } else {
                (args[1].clone(), "-color".to_string())
            }
        }
        _ => (args[1].clone(), args[2].clone()),
    };

    if mode == "-h" {
        println!("{}", "版权所有 (c) 2024 by Tianze".yellow());
        println!("{}", "Usage: ls [path] [mode]".green());
        println!("{}", "path: 指定要列出的目录路径，默认为当前目录".blue());
        println!("{}", "mode: 指定显示模式，支持以下选项：".blue());
        println!("{}", "  -color    彩色显示文件和文件夹（默认）".purple());
        println!("{}", "  -ln       每行显示一个文件或文件夹".purple());
        println!("{}", "  -nocolor  不使用颜色显示文件和文件夹".purple());
        println!("{}", "  -icon     显示图标和颜色".purple());
        println!("{}", "  -iconln   显示图标，每行一个".purple());
        println!("{}", "  -h        显示帮助信息".purple());
        println!("{}", "  -v        显示版本信息".purple());
        println!("{}", "  -e        打开资源管理器".purple());
        return;
    } else if mode == "-e" {
        let current_dir = env::current_dir().expect("无法获取当前目录");
        let path_str = current_dir.to_str().expect("路径包含无效字符");
        Command::new("explorer.exe")
            .arg(path_str)
            .spawn()
            .expect("无法启动资源管理器");
        return;
    } else if mode == "-v" {
        println!("{}", "版权所有 (c) 2024 by Tianze".yellow());
        println!("{}", "ls for windows: veresion 1.0.0".red());
        return;
    }

    let result = match read_directory(path.as_str()) {
        Ok(v) => v,
        Err(_e) => {
            println!("{}", "Error: 找不到文件夹".red());
            return;
        }
    };

    for item in result {
        match mode.as_str() {
            "-color" => item.print(),
            "-ln" => item.println(),
            "-nocolor" => item.nocolor_print(),
            "-icon" => item.icon_print(),
            "-iconln" => item.icon_println(),
            _ => item.icon_print(),
        }
    }
}
