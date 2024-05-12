pub fn condition1() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

pub fn get_month_name(month: u8) {
    let name = match month {
        1 | 2 | 3 => "春",
        4 | 5 | 6 => "夏",
        7 | 8 | 9 => "秋",
        10 | 11 | 12 => "冬",
        _ => "不明",
    };
    println!("{}月は{}です", month, name);
}
