enum Color {
    Red,
    White,
    Black,
}

fn show_color() {
    let color = Color::Red;
    match color {
        Color::Red => println!("is red"),
        Color::White | Color::Black => println!("white or black"),
        _ => println!("未知"),
    }
}

// 赋值
fn set_val() {
    let color = Color::Red;
    let num = match color {
        Color::Red => 1,
        Color::Black => 2,
        Color::White => 3,
        _ => 0,
    };

    println!("num is {}", num);
}

pub fn test_matchs() {
    show_color();
    set_val();
}
