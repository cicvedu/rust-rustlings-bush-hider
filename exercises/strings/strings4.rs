// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// ## common String&str : 常见的字符串类型和字符串切片类型的转换

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));                 // &str -> String
    string("rust is fun!".to_owned());          // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    string_slice("nice weather".into());        // String -> &str  .as_str
    string(format!("Interpolation {}", "Station"));             // format! -> String
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());                      // trim -> &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace -> String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());            // to_lowercase -> String
}
