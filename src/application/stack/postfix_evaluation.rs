use crate::Stack;

fn binary_expression(o: &char, a: i32, b: i32) -> i32 {
  println!("{a} {o} {b}");
  match o {
    '+' => a + b,
    '-' => a - b,
    '*' => a * b,
    '/' => a / b,
    '%' => a % b,
    _ => 0,
  }
}

pub fn postfix_evaluation(expression: &str) -> Option<i32> {
  let mut stack = Stack::new(expression.len());

  for c in expression.chars() {
    match c {
      '0'..'9' => stack.push(c.to_digit(10).unwrap() as i32),
      '+' | '-' | '*' | '/' | '%' => {
        if let Some(exp1) = stack.pop() {
          if let Some(exp2) = stack.pop() {
            let res = binary_expression(&c, exp2, exp1);
            stack.push(res);
          }
        }
      }
      _ => {}
    }
  }

  stack.pop()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_postfix_evaluation() {
    assert_eq!(postfix_evaluation("123*+5-"), Some(2));
  }
}
