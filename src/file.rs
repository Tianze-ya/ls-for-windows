use crate::item::{Item, ItemList};
use std::env;
use std::fs;

pub fn read_directory(path: &str, mode: String) -> std::io::Result<ItemList> {
    let entries = fs::read_dir(path)?;
    let mut olditems = ItemList::new(mode.clone());
    let mut diritems = ItemList::new(mode.clone());
    let mut hideitems = ItemList::new(mode.clone());
    let mut items = ItemList::new(mode.clone());

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            diritems.add(Item::new(path, mode.clone()));
        } else if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
            hideitems.add(Item::new(path, mode.clone()));
        } else {
            olditems.add(Item::new(path, mode.clone()));
        }
    }
    olditems
        .get_mut_items()
        .sort_by(|a, b| a.suffix.cmp(&b.suffix));
    items.get_mut_items().extend(diritems.get_copy_items());
    items.get_mut_items().extend(hideitems.get_copy_items());
    items.get_mut_items().extend(olditems.get_copy_items());

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

pub fn set_config(mode: String) {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().unwrap();
    let config_path = dir.join("config.txt").to_str().unwrap().to_string();
    fs::write(&config_path, mode).expect("无法修改配置文件");
}