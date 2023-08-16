// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// ## reduce & flod : without loops

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // recursion
    // match num {
    //    0 | 1 => 1,
    //    _ => num * factorial( num - 1 ),
    //}

    // (1 .. num + 1) : like a iter()
    // ## use reduce : '聚合' 相同类型的数值
    (1 .. num + 1).reduce(|acc, x| acc * x).unwrap_or(1) // ## unwrap_or(x) : 解析 None 为 x
    // ## another : use flod : 初始化一个结果，并在遍历中进行 '聚合' 逻辑
    // (1 .. num + 1).fold(1, |acc, x| acc * x) // firstly, acc = 1; in iter(for each x), acc *= x;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
