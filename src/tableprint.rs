use prettytable::{Cell, Row, Table};
use terminal_size::{terminal_size, Width};
use unicode_width::UnicodeWidthStr; // 用于准确计算字符串在终端显示的宽度

pub fn tableprint(files:Vec<String>) {
    // 获取终端宽度
    let term_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80 // 默认终端宽度
    };

    // 计算最大文件名宽度（考虑Unicode字符）
    let max_width = files
        .iter()
        .map(|s| UnicodeWidthStr::width(s.as_str()))
        .max()
        .unwrap_or(0)
        + 2; // 添加一些间距

    // 计算可以显示多少列
    let num_cols = (term_width / max_width).max(1);
    let num_rows = (files.len() + num_cols - 1) / num_cols; // 向上取整

    let mut table = Table::new();
    // 不显示表格边框和分隔线，更接近 ls 的外观
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

    // 按行添加数据
    for row in 0..num_rows {
        let mut cells = Vec::new();
        for col in 0..num_cols {
            let index = row + col * num_rows;
            if index < files.len() {
                // 左对齐，并使用计算出的宽度进行填充
                cells.push(Cell::new(&format!("{:<width$}", files[index], width = max_width)));
            } else {
                // 添加空格占位符以保持对齐
                cells.push(Cell::new(&" ".repeat(max_width)));
            }
        }
        table.add_row(Row::new(cells));
    }

    table.printstd();
}