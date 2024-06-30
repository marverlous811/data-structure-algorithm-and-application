pub fn bubble_sort(arr: &mut Vec<i32>) {
  let len = arr.len();
  if len < 2 {
    return;
  }

  for i in 0..len {
    for j in 0..len - i - 1 {
      if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bubble_sort() {
    let mut arr = vec![3, 2, 1, 5, 4];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
  }
}
