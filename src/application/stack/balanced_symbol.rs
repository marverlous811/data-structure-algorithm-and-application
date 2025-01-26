use crate::data_structure::Stack;

pub fn is_balanced_symbol(expression: &str) -> bool {
  let mut stack = Stack::new(expression.len());
  for c in expression.chars() {
    match c {
      '(' | '[' | '{' => stack.push(c),
      ')' => {
        let top = stack.pop();
        match top {
          Some('(') => {}
          _ => return false,
        }
      }
      ']' => {
        let top = stack.pop();
        match top {
          Some('[') => {}
          _ => return false,
        }
      }
      '}' => {
        let top = stack.pop();
        match top {
          Some('{') => {}
          _ => return false,
        }
      }
      _ => {}
    }
  }

  stack.is_empty()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_balanced_symbol() {
    assert_eq!(is_balanced_symbol("()"), true);
    assert_eq!(is_balanced_symbol("()[]{}"), true);
    assert_eq!(is_balanced_symbol("(]"), false);
    assert_eq!(is_balanced_symbol("([)]"), false);
    assert_eq!(is_balanced_symbol("{[]}"), true);
  }

  #[test]
  fn test_is_balanced_symbol_with_empty_string() {
    assert_eq!(is_balanced_symbol(""), true);
  }

  #[test]
  fn test_is_balanced_symbol_with_invalid_expression() {
    assert_eq!(is_balanced_symbol(")"), false);
    assert_eq!(is_balanced_symbol("]"), false);
    assert_eq!(is_balanced_symbol("}"), false);
  }

  #[test]
  fn test_real_world_expression() {
    assert_eq!(is_balanced_symbol("let a = (1 + 2) * 3;"), true);
    assert_eq!(is_balanced_symbol("let a = (1 + 2 * 3;"), false);
  }
}
