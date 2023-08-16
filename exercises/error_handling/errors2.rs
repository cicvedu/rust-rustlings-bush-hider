// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

// ## ?? match Result
// parse() 方法返回 Result 类型，因为它不是总能把字符串解析成指定的类型，Result 表示可能的失败
// 通过 match 匹配 parse() 方法的 Result : Ok or Err，并进行下一步的处理


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();
    // 方法一：使用 match 匹配 parse 结果: Ok or Err
    match qty {
        Ok(v) => Ok(v * cost_per_item + processing_fee),
        Err(e) => Err(e),
    }

    // 方法二：使用 if let 语句尝试解析 Result
    // if let Ok(v) = qty {
    //    Ok(v * cost_per_item + processing_fee)
    //}
    //else {
    //    qty // Err(e)
    //}

    // 方法三：更简洁地使用`？`进行错误处理：成功则解释为 Ok，否则返回默认的 Err
    // let qty = item_quantity.parse::<i32>()?; // 解释成功，则 qty 为 i32
    // Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
