use prettytable::{Cell, Row, Table};
use terminal_size::{Width, terminal_size};
use unicode_width::UnicodeWidthStr;
use crate::item::ItemList;
use regex::Regex;

// 去除ANSI转义字符的正则表达式
fn strip_ansi_codes(text: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(text, "").to_string()
}

pub fn tableprint(itemlist: &ItemList) {
    // 获取终端宽度
    let term_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize -25
    } else {
        80 // 默认终端宽度
    };
    
    let items = itemlist.get_items();
    // 计算最大文件名宽度（考虑Unicode字符），去除ANSI转义字符
    let max_width = items
        .iter()
        .map(|item| {
            let clean_text = strip_ansi_codes(item.get_text());
            UnicodeWidthStr::width(clean_text.as_str())
        })
        .max()
        .unwrap_or(0)
        + 2; // 添加一些间距

    println!("Terminal width: {}, max width: {}", term_width, max_width);

    // 计算可以显示多少列
    let num_cols = (term_width / max_width).max(1);
    let num_rows = (items.len() + num_cols - 1) / num_cols; // 向上取整
    println!("Number of columns: {}, rows: {}", num_cols, num_rows);

    let mut table = Table::new();
    // 不显示表格边框和分隔线，更接近 ls 的外观
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

    // 按行添加数据
    for row in 0..num_rows {
        let mut cells = Vec::new();
        for col in 0..num_cols {
            let index = row * num_cols + col;
            if index >= items.len() {
                break;
            }
            // 左对齐，并使用计算出的宽度进行填充
            if itemlist.get_icon() {
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

    table.printstd();
}
