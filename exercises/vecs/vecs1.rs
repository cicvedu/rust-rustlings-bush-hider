// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// ## vec init

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // 方法一：Vec::new()
    // 由于类型推断，编译器可以根据插入元素推断类型。
    // let mut v = Vec::new();
    // for i in 0..4 {
    //     v.push(a[i]);
    // }

    // 方法二：用 vec![] 宏来初始化
    let v = vec![10,20,30,40];

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
