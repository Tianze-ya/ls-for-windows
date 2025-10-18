use crate::item::ItemList;
use prettytable::{Cell, Row, Table};
use regex::Regex;
use terminal_size::{Width, terminal_size};
use unicode_width::UnicodeWidthStr;

pub fn tableprint(itemlist: &ItemList) {
    // 获取终端宽度
    let term_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80 // 默认终端宽度
    };
    let items = itemlist.get_items();
    let tabwidth = 4;
    let mut table = Table::new();
    // 不显示表格边框和分隔线，更接近 ls 的外观
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    // 计算最大文件名宽度
    let max_width = items
        .iter()
        .map(|item| {
            let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
            let clean_text = re.replace_all(item.get_name(), "").to_string();
            UnicodeWidthStr::width(clean_text.as_str())
        })
        .max()
        .unwrap_or(0)
        + tabwidth; // 添加一些间距
    // 计算可以显示多少列
    let num_cols = (term_width / max_width).max(1); // 至少一列
    let num_rows = (items.len() + num_cols - 1) / num_cols; // 向上取整

    //println!("Terminal width: {}, max width: {}", term_width, max_width);
    //println!("Number of columns: {}, rows: {}", num_cols, num_rows);

    // 按行添加数据
    for row in 0..num_rows {
        let mut cells = Vec::new();
        for col in 0..num_cols {
            let index = row * num_cols + col;
            if index >= items.len() {
                break;
            }
            // 左对齐，并使用计算出的宽度进行填充
            if itemlist.is_icon() {
                cells.push(Cell::new(&format!(
                    "{} {:<width$}",
                    items[index].get_icon(),
                    items[index].get_text(),
                    width = max_width
                )));
            } else {
                cells.push(Cell::new(&format!(
                    "{:<width$}",
                    items[index].get_text(),
                    width = max_width
                )));
            }
        }

        table.add_row(Row::new(cells));
    }

    {/*
    let mut count = 1;
    for item in items {
        if itemlist.get_icon() {
            print!(
                "{} {:<width$}{:<tabwidth$}",
                item.get_icon(),
                item.get_text(),
                " ",
                width = max_width,
                tabwidth = tabwidth
            );
        } else {
            print!(
                "{:<width$}{:<tabwidth$}",
                item.get_text(),
                " ",
                width = max_width,
                tabwidth = tabwidth
            );
        }
        if count % num_cols == 0 {
            println!();
        }
        count += 1;
    }
    */}

    table.printstd();
}
