// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

// 考察生命周期

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&data); // 解法1，之后的data还可以使用只发生了借用
    string_uppercase_none(data); // 转移了所有权，data变量发生了drop
                                 //之后不可以在调用data  eg：println!("{}", data)，报错
}

// Should not take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    let data = &data.to_uppercase();

    println!("{}", data);
}

fn string_uppercase_none(mut data: String) {
    // 转移了所有权，data变量发生了drop
    data = data.to_uppercase();
    println!("{}", data)
}
