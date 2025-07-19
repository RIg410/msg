use msg::*;

fn main() {
    let table1 = Element::Table(TableNode {
        headers: vec![
            TableCell {
                content: vec![Element::text("Товар")],
                align: CellAlign::Left,
                colspan: 1,
                rowspan: 1,
            },
            TableCell {
                content: vec![Element::text("Количество")],
                align: CellAlign::Center,
                colspan: 1,
                rowspan: 1,
            },
            TableCell {
                content: vec![Element::text("Цена")],
                align: CellAlign::Right,
                colspan: 1,
                rowspan: 1,
            },
        ],
        rows: vec![
            TableRow {
                cells: vec![
                    TableCell {
                        content: vec![Element::text("Яблоки")],
                        align: CellAlign::Left,
                        colspan: 1,
                        rowspan: 1,
                    },
                    TableCell {
                        content: vec![Element::text("10")],
                        align: CellAlign::Center,
                        colspan: 1,
                        rowspan: 1,
                    },
                    TableCell {
                        content: vec![Element::text("150₽")],
                        align: CellAlign::Right,
                        colspan: 1,
                        rowspan: 1,
                    },
                ],
            },
            TableRow {
                cells: vec![
                    TableCell {
                        content: vec![Element::text("Груши")],
                        align: CellAlign::Left,
                        colspan: 1,
                        rowspan: 1,
                    },
                    TableCell {
                        content: vec![Element::text("5")],
                        align: CellAlign::Center,
                        colspan: 1,
                        rowspan: 1,
                    },
                    TableCell {
                        content: vec![Element::text("200₽")],
                        align: CellAlign::Right,
                        colspan: 1,
                        rowspan: 1,
                    },
                ],
            },
            TableRow {
                cells: vec![
                    TableCell {
                        content: vec![Element::bold(vec![Element::text("Итого")])],
                        align: CellAlign::Left,
                        colspan: 1,
                        rowspan: 1,
                    },
                    TableCell {
                        content: vec![Element::bold(vec![Element::text("15")])],
                        align: CellAlign::Center,
                        colspan: 1,
                        rowspan: 1,
                    },
                    TableCell {
                        content: vec![Element::bold(vec![Element::text("350₽")])],
                        align: CellAlign::Right,
                        colspan: 1,
                        rowspan: 1,
                    },
                ],
            },
        ],
        style: TableStyle::Unicode,
        rules: vec![],
    });

    let generator = Generator::new(ParseMode::MarkdownV2);

    println!("=== Таблица Unicode ===");
    println!("{}", generator.generate(&table1).unwrap());

    let table2 = match table1.clone() {
        Element::Table(mut t) => {
            t.style = TableStyle::Ascii;
            Element::Table(t)
        }
        _ => unreachable!(),
    };

    println!("\n=== Таблица ASCII ===");
    println!("{}", generator.generate(&table2).unwrap());

    let table3 = match table1.clone() {
        Element::Table(mut t) => {
            t.style = TableStyle::Minimal;
            Element::Table(t)
        }
        _ => unreachable!(),
    };

    println!("\n=== Таблица Minimal ===");
    println!("{}", generator.generate(&table3).unwrap());

    let table4 = match table1 {
        Element::Table(mut t) => {
            t.style = TableStyle::Compact;
            Element::Table(t)
        }
        _ => unreachable!(),
    };

    println!("\n=== Таблица Compact ===");
    println!("{}", generator.generate(&table4).unwrap());
}
