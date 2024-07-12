pub fn merge_two_sorted_array(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
  let mut retval = vec![];

  let mut i = 0;
  let mut j = 0;
  loop {
    if i >= left.len() || j >= right.len() {
      break;
    }

    if left[i] <= right[j] {
      //   println!("left <= right");
      retval.push(left[i]);
      i += 1;
    } else {
      //   println!("left > right");
      retval.push(right[j]);
      j += 1;
    }
  }

  //   println!("{i} {j}");
  while i < left.len() {
    retval.push(left[i]);
    i += 1;
  }

  while j < right.len() {
    retval.push(right[j]);
    j += 1;
  }

  retval
}

#[cfg(test)]
mod tests {
  use crate::merge_two_sorted_array;

  #[test]
  fn merge_array_with_left_larger() {
    let left = vec![1, 2, 5, 7];
    let right = vec![3, 4, 6];

    let retval = merge_two_sorted_array(&left, &right);
    assert_eq!(retval, vec![1, 2, 3, 4, 5, 6, 7]);
  }

  #[test]
  fn merge_array_with_right_larger() {
    let left = vec![3, 4, 6];
    let right = vec![1, 2, 5, 7];

    let retval = merge_two_sorted_array(&left, &right);
    assert_eq!(retval, vec![1, 2, 3, 4, 5, 6, 7]);
  }

  #[test]
  fn merge_array_with_same_len() {
    let left = vec![3, 4, 6];
    let right = vec![1, 2, 5];

    let retval = merge_two_sorted_array(&left, &right);
    assert_eq!(retval, vec![1, 2, 3, 4, 5, 6]);
  }
}
