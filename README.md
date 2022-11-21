# RPN(Reverse Polish Notation) Calc

Reverse Polish Notation (RPN) Calc.

## Installation

```
cargo add rpn_lib
```

## Example

```
fn main() {
    let src = String::from("1 2 + 3 * ");
    let a = rpn_lib::eval(src).unwrap();
    println!("{}", a); // →9
}
```

## Example2
```
fn main() {
    let result = rpn_lib::eval_from_str("1 2 3 * +");
    println!("{}", result); // →7
}
```