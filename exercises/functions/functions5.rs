// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// function bolck

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// solution 1
// fn square(num: i32) -> i32 {
//     return num * num;
// }

// solution 2 
fn square(num: i32) -> i32 {
    num * num
}
