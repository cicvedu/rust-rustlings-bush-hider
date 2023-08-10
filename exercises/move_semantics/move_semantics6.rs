// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD
// ##??
// The first problem is that `get_char` is taking ownership of the string.
// So `data` is moved and can't be used for `string_uppercase`
fn main() {

    let data = "Rust is great!".to_string();
    // move occurs because `data` has type `String`, which does not implement the `Copy` trait
    // 需要拷贝函数形参，然而 String 无 `Copy` trait
    // get_char(data);
    get_char(data.clone());
=======
// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
<<<<<<< HEAD
    // temporary value is freed at the end of this statement
    // creates a temporary value which is freed while still in use
    // 函数中重新赋值 data:&String，因为函数占有对原值的所有权，所以这会释放原值，然而原值还在函数外使用
    // data = &data.to_uppercase();
    let data2 = &data.to_uppercase();
    println!("{}", data2);
=======
    data = &data.to_uppercase();

    println!("{}", data);
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc
}
