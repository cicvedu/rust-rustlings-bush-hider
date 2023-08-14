// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// ## ref when match : 在通过 let 绑定来进行模式匹配或解构时，ref 关键字可用来创建结构体/元组的字段的引用。
// 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号，在解构一个结构体时 `ref` 同样有效

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    //  a partial move happened in the `match` statement
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        None => panic!("no match!"),
    }
    // y : value used here after partial move
    y; // Fix without deleting this line.
}
