// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// if let & while let

// `if let Some(i) = number` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行语句块（`{}`）
// 同样，可以用 if let 匹配任何枚举值：if let Foo::Bar = a 表示变量 a 匹配到了 Foo::Bar

// while let : 当 `let` 将 `optional` 解构成 `Some(i)` 时，就执行语句块（`{}`）。否则就 `break`

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.

        while let Some(optional) = optional_integers.pop() {
            // vector.pop also adds another layer of Option<T>
            if let Some(integer) = optional {
                assert_eq!(integer, cursor);
                cursor -= 1; // ?? 为什么 cursor 不能放在 if let 语句块外面 
            }
        }

        assert_eq!(cursor, 0);
    }
}
