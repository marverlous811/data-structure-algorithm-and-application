pub fn quick_sort(arr: &mut Vec<i32>) {
  let len = arr.len();
  if len < 2 {
    return;
  }

  let pivot = len / 2;
  let pivot_val = arr[pivot];
  let mut left = vec![];
  let mut right = vec![];
  for i in 0..len {
    if i == pivot {
      continue;
    }
    if arr[i] < pivot_val {
      left.push(arr[i]);
    } else {
      right.push(arr[i]);
    }
  }

  quick_sort(&mut left);
  quick_sort(&mut right);

  arr.clear();
  arr.extend(left);
  arr.push(pivot_val);
  arr.extend(right);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_quick_sort() {
    let mut arr = vec![3, 2, 1, 5, 4];
    quick_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
  }
}
