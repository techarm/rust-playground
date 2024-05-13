pub fn loop_for_num_total(start: u32, end: u32) {
    let mut total = 0;
    let mut count = start;
    loop {
        if count > end {
            break;
        }
        total += count;
        count += 1;
    }
    println!("1から{}までの合計は{}です", end, total);
}

pub fn while_for_num_total(start: u32, end: u32) {
    let mut total = 0;
    let mut count = start;
    while count <= end {
        total += count;
        count += 1;
    }
    println!("1から{}までの合計は{}です", end, total);
}

pub fn for_for_num_total(start: u32, end: u32) {
    let mut total = 0;
    for count in start..=end {
        total += count;
    }
    println!("1から{}までの合計は{}です", end, total);
}

pub fn list_num_sum(numbers: Vec<i32>) {
    let mut total = 0;
    for number in numbers.iter() {
        total += number;
    }
    println!("合計は{}です", total);
}
