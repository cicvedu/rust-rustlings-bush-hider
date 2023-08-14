// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

// ## string : &str ( 字符串引用作为函数参数传递 )
// Rust 中有两种字符串类型：String 和 &str。
// String 被存储为由字节组成的 vector（Vec<u8>），但保证了它一定是一个有效的 UTF-8 序列。
// String 是堆分配的，可增长的，且不是零结尾的（null terminated）。
// &str 是一个总是指向有效 UTF-8 序列的切片（&[u8]），
// &str 可用来查看 String 的内容，就如同 &[T] 是 Vec<T> 的全部或部分引用。
// &str.to_string -> String

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
