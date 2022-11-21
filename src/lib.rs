/// Eval RPN
/// # Example
/// ```
/// let src = String::from("1 2 + 3 *");
/// let result = rpn_lib::eval(src).unwrap();
/// println!("{}", result) // →9
/// ```
/// # Example2
/// ```
/// let result = rpn_lib::eval_from_str("1 2 3 * +");
/// println!("{}", result); // →7
/// ```
pub fn eval(src: String) -> Result<f64, String> {
    // 計算用のスタック
    let mut stack: Vec<f64> = vec![];

    // 式を空白で分割して繰り返し計算
    src.split_whitespace()
                .filter(|token| token != &"")
                .map(str::trim)
                .for_each(|token| exec_eval(token, &mut stack));

    // 結果を返却
    if stack.len() == 0 {
        return Err(format!("no result"));
    }
    if stack.len() > 1 {
        return Err(format!("too many value in stack"));
    }
    Ok(stack.pop().unwrap_or(0.0))
}

// より手軽に使える方法を提供
pub fn eval_from_str(src: &str) -> String {
    match eval(String::from(src)) {
        Ok(v) => format!("{}", v),
        Err(e) => format!("[ERROR]: {}", e),
    }
}

fn exec_eval(token: &str, stack: &mut Vec<f64>) {
    // 数値の場合は、スタックにPUSH
    match token.parse::<f64>() {
        Ok(v) => {
            stack.push(v);
            return;
        },
        Err(_) => 0.0,
    };

    // 演算子なら直前2つをPOPして、計算結果をPUSH
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    match token {
        "+" => stack.push(a + b),
        "-" => stack.push(a - b),
        "*" => stack.push(a * b),
        "/" => stack.push(a / b),
        _ => panic!("invalid operator: {}", token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // test for eval
        assert_eq!(eval(String::from("2 2 +")).unwrap(), 4.0);
        assert_eq!(eval(String::from("2 3 *")).unwrap(), 6.0);
        assert_eq!(eval(String::from("6 2 /")).unwrap(), 3.0);
        assert_eq!(eval(String::from("6 3 - 1 -")).unwrap(), 2.0);

        // test for eval_from_str
        assert_eq!(eval_from_str("1 2 3 + +"), "6".to_string());
        assert_eq!(eval_from_str("1 2 3 * +"), "7".to_string());
    }
}
