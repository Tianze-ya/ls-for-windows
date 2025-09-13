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
            ("󰘓", name.dimmed()) // 隐藏文件图标
        } else {
            match suffix.as_str() {
                "dir" => ("󰉋", name.blue()),           // 文件夹图标
                "rs" => ("", name.green()),           // Rust文件图标
                "md" => ("", name.normal()),          // Markdown文件图标
                "toml" => ("", name.yellow()),        // TOML文件图标
                "lock" => ("󰈡", name.red()),           // Lock文件图标
                "gitignore" => ("󰊢", name.magenta()),  // Gitignore文件图标
                "json" => ("", name.bright_yellow()), // JSON文件图标
                "txt" => ("󰈙", name.cyan()),           // TXT文件图标
                "html" => ("", name.red()),           // HTML文件图标
                "css" => ("", name.blue()),           // CSS文件图标
                "js" => ("", name.yellow()),          // JavaScript文件图标
                "py" => ("", name.blue()),            // Python文件图标
                "exe" => ("", name.red()),            // 可执行文件图标
                "bat" => ("", name.yellow()),         // 批处理文件图标
                "sh" => ("", name.green()),           // Shell脚本文件图标
                _ => ("󰈔", name.normal()),             // 默认文件图标
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
        print!("{} {}  ", self.icon, self.colored_name);
    }

    #[allow(dead_code)]
    fn icon_println(&self) {
        println!("{} {}  ", self.icon, self.colored_name);
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
        println!("{}","版权所有 (c) 2024 by Tianze".yellow());
        println!("{}","Usage: ls [path] [mode]".green());
        println!("{}","path: 指定要列出的目录路径，默认为当前目录".blue());
        println!("{}","mode: 指定显示模式，支持以下选项：".blue());
        println!("{}","  -color    彩色显示文件和文件夹（默认）".purple());
        println!("{}","  -ln       每行显示一个文件或文件夹".purple());
        println!("{}","  -nocolor  不使用颜色显示文件和文件夹".purple());
        println!("{}","  -icon     显示图标和颜色".purple());
        println!("{}","  -iconln   显示图标，每行一个".purple());
        println!("{}","  -h        显示帮助信息".purple());
        return;
    }else if mode == "-e"{
        let current_dir = env::current_dir().expect("无法获取当前目录");
        let path_str = current_dir.to_str().expect("路径包含无效字符");
        Command::new("explorer.exe")
            .arg(path_str)
            .spawn()
            .expect("无法启动资源管理器");
        return;
    }else if mode == "-v"{
        println!("{}","版权所有 (c) 2024 by Tianze".yellow());
        println!("{}","ls for windows: veresion 1.0.0".red());
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
