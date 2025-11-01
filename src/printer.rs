use crate::item::{Item, ItemList};
use terminal_size::{Width, terminal_size};

struct Printer {
    tabwidth: usize,
    line: usize,
    items: Vec<Vec<String>>,
}

impl Printer {
    pub fn new(tabwidth: usize) -> Printer {
        Printer {
            tabwidth,
            line: 0,
            items: vec![vec![]],
        }
    }

    pub fn new_line(&mut self) {
        self.items.push(vec![]);
        self.line += 1;
    }

    pub fn add_item(&mut self, item: String) {
        self.items[self.line].push(item);
    }

    pub fn print(&self) {
        let max_columnumn: usize = self.items.iter().map(|x| x.len()).max().unwrap_or(0);
        for i in 0..max_columnumn {
            for j in 0..self.line {
                if i >= self.items[j].len() {
                    continue;
                }
                let text: &String = &self.items[j][i];
                print!("{text}{}", " ".repeat(self.tabwidth));
            }
            println!("");
        }
    }
}

pub fn stdprint(itemlist: &ItemList) {
    // 获取终端宽度
    let term_width: usize = get_terminal_width();
    // 获取所有项目
    let items: &Vec<Item> = itemlist.get_items();
    // 设置tab宽度
    let tabwidth: usize = 2;
    // 获取项目长度
    let items_len: usize = items.len();
    // 获取中间宽度 {{{{{WARNING}}}}}
    let middle_width: usize = get_middle_width(items, tabwidth);
    // 计算可以显示多少列
    let num_columns: usize = (term_width / middle_width).max(1);
    // 初始化打印,变量
    let mut printer: Printer = Printer::new(tabwidth);
    let mut max_width: usize;
    let mut this_column_len: usize;
    let mut this_column_start_index: usize = 0;
    let layout: Vec<usize> = get_layout_columns(items_len, num_columns);
    for this_column in 1..num_columns + 1 {
        // 获取当前列有多少个元素
        this_column_len = layout[this_column - 1];
        // 计算最大文件名宽度
        max_width = get_max_width(items, this_column_start_index, this_column_len, tabwidth);

        for k in 0..this_column_len {
            let index = k + this_column_start_index;

            // 分别输出
            let name_len = max_width - items[index].get_name().len();
            let text = if itemlist.is_icon() {
                format!(
                    "{} {}{}",
                    items[index].get_icon(),
                    items[index].get_text(),
                    " ".repeat(name_len)
                )
            } else {
                format!("{}{}", items[index].get_text(), " ".repeat(name_len))
            };
            printer.add_item(text);
        }
        // 换行
        printer.new_line();
        // 更新起始索引
        this_column_start_index += this_column_len;
    }
    printer.print();
}

fn get_terminal_width() -> usize {
    let width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        // 默认终端宽度
        80
    };
    return width;
}

fn get_layout_columns(length: usize, column: usize) -> Vec<usize> {
    let mut layout: Vec<usize> = vec![];
    let mut left: usize = length;
    for _ in 0..column {
        layout.push(0);
    }
    while left != 0 {
        for i in 0..column {
            if left == 0 {
                break;
            }
            layout[i] += 1;
            left -= 1;
        }
    }

    return layout;
}

fn get_middle_width(items: &Vec<Item>, tabwidth: usize) -> usize {
    items
        .iter()
        .map(|item| item.get_text().len() + tabwidth + 2)
        .sum::<usize>()
        / items.len()
}

fn get_max_width(
    items: &Vec<Item>,
    this_column_start_index: usize,
    this_column_len: usize,
    tabwidth: usize,
) -> usize {
    items[this_column_start_index..this_column_start_index + this_column_len]
        .iter()
        .map(|item| item.get_name().len())
        .max()
        .unwrap_or(0)
        + tabwidth
}
