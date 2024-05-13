mod condition;
mod loop_control;

fn main() {
    println!("--- if式を使う ---");
    condition::condition1();

    println!("\n--- match式を使う ---");
    condition::get_month_name(1);

    println!("\n--- loop式を使う ---");
    loop_control::loop_for_num_total(1, 100);

    println!("\n--- while式を使う ---");
    loop_control::while_for_num_total(1, 100);

    println!("\n--- for式を使う ---");
    loop_control::for_for_num_total(1, 100);

    println!("\n--- for式を使ってリストの合計を求める ---");
    loop_control::list_num_sum(vec![1, 2, 3, 4, 5]);
}
