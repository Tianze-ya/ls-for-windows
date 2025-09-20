use crate::item::{Item, ItemList};
use std::env;
use std::fs;

pub fn read_directory(path: &str, mode: String) -> std::io::Result<ItemList> {
    let entries = fs::read_dir(path)?;
    let mut items = ItemList::new(mode.clone());

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        items.add(Item::new(path, mode.clone()));
    }

    return Ok(items);
}

pub fn get_config() -> String {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().unwrap();
    let config_path = dir.join("config.txt").to_str().unwrap().to_string();
    let mode = match fs::read_to_string(&config_path) {
        Ok(v) => v,
        Err(_e) => {
            fs::write(&config_path, "-color").expect("无法创建配置文件");
            return "-color".to_string();
        }
    };
    return mode;
}
