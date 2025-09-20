mod file;
mod item;
mod tableprint;
use colored::*;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (path, mode) = match args.len() {
        1 => ("./".to_string(), file::get_config()),
        2 => {
            if args[1].starts_with("-") {
                ("./".to_string(), args[1].clone())
            } else {
                (args[1].clone(), file::get_config())
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

    let items = match file::read_directory(path.as_str(),mode) {
        Ok(v) => v,
        Err(_e) => {
            println!("{}", "Error: 找不到文件夹".red());
            return;
        }
    };

    items.print();
}
