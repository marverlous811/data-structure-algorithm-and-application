use crate::data_structure::Stack;

pub fn find_span_brute_force(arr: &[i32]) -> Vec<i32> {
  let mut result = vec![1; arr.len()];
  for i in 0..arr.len() {
    let mut span = 1;
    let mut j = 1;
    while j <= i && arr[i] > arr[i - j] {
      span += 1;
      j += 1;
    }
    result[i] = span;
  }

  result
}

pub fn find_span_with_stack(arr: &[i32]) -> Vec<i32> {
  let mut stack = Stack::<i32>::new(arr.len());
  let mut result = vec![1; arr.len()];
  let mut p: i32;
  for i in 0..arr.len() {
    while !stack.is_empty() {
      let top = if let Some(top) = stack.top() {
        *top
      } else {
        break;
      };

      if arr[i] < arr[top as usize] {
        break;
      }

      stack.pop();
    }

    p = if let Some(top) = stack.top() {
      *top
    } else {
      -1
    };

    result[i] = i as i32 - p;
    stack.push(i as i32);
  }

  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_find_span_brute_force() {
    assert_eq!(find_span_brute_force(&[6, 3, 4, 5, 2]), vec![1, 1, 2, 3, 1]);
  }

  #[test]
  fn test_find_span_with_stack() {
    assert_eq!(find_span_with_stack(&[6, 3, 4, 5, 2]), vec![1, 1, 2, 3, 1]);
  }
}
