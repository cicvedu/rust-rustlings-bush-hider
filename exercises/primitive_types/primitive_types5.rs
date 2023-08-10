// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// ## tuple init & deconstruct
// 元组是一个可以包含各种类型值的组合。元组使用括号 () 来构造（construct），
// 元组自身是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是每个元素的类型。
// 函数可以使用元组来返回多个值，因为元组可以拥有任意多个值。
// 元组可以被解构（deconstruct），从而将数据绑定给变量。

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name,age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
