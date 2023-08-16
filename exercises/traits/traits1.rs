// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.


// ## 特征 trait 与泛型 generics 相关联，从而表述具有某种特征（接口）的类型，可以称之为特征的实现 impl A + B
// ## 特征约束(trait bound)
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // 特征的实现作为参数
// pub fn notify(item: &(impl Summary + Display)) {}  // 参数实现 Summary 和 Display 两个特征
// pub fn notify<T: Summary>(item1: &T, item2: &T) {} // 两个参数具有同一类型的特征（接口）
// 泛型类型 T 说明了 item1 和 item2 必须拥有同样的类型，同时 T: Summary 说明了 T 必须实现 Summary 特征。
// 多重特征和多重参数：
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}

// ##特征对象
// (1)函数返回某种特征的实现（impl Trait），然而并不支持多种不同的具体类型返回（如类A、B都实现了同一特征，不能返回 impl）
// (2)在拥有继承的语言中，可以定义一个名为 Component 类，该类上有一个 draw 方法。
// 其他的类比如 Button、Image 和 SelectBox 会从 Component 派生并因此继承 draw 方法，它们各自都可以覆盖 draw 方法来定义自己的行为.
// 但是框架会把所有这些类型当作是 Component 的实例，并在其上调用 draw。不过 Rust 并没有继承，我们得另寻出路。
// 通过特征对象来表示类似 Component 的具有某种特征（接口）的类型。
// 可以通过 &dyn 引用或者 Box<dyn Trait> 智能指针（值被强制分配到堆上的引用）的方式来创建特征对象;
// dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self{
        format!("{}{}", self.as_str(), "Bar")
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
