use crate::Stack;

fn precedence(c: char) -> i32 {
  match c {
    '+' | '-' => 12,
    '*' | '/' | '%' => 13,
    _ => 0,
  }
}

pub fn postfix_conversion(expression: &str) -> String {
  let mut stack = Stack::new(expression.len());
  let mut result = String::new();
  for c in expression.chars() {
    match c {
      '+' | '-' | '*' | '/' | '%' | '(' => {
        while let Some(top) = stack.pop() {
          if top == '(' || precedence(top) < precedence(c) {
            stack.push(top);
            break;
          }
          result.push(top);
        }
        stack.push(c);
      }
      ')' => {
        while let Some(top) = stack.pop() {
          if top == '(' {
            break;
          }
          result.push(top);
        }
      }
      _ => result.push(c),
    }
  }

  while let Some(top) = stack.pop() {
    result.push(top);
  }

  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_postfix_conversion() {
    assert_eq!(postfix_conversion("(a+b)"), "ab+".to_string());
    assert_eq!(postfix_conversion("(a+b)*c"), "ab+c*".to_string());
    assert_eq!(postfix_conversion("a+b*c"), "abc*+".to_string());
    assert_eq!(postfix_conversion("a+b*c+d"), "abc*+d+".to_string());
    assert_eq!(postfix_conversion("a+b*c+d/e"), "abc*+de/+".to_string());
    assert_eq!(postfix_conversion("((a+b)*c)-d"), "ab+c*d-".to_string());
  }
}
