use crate::merge_two_sorted_array;

pub fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
  let left_arr_len = mid - left + 1;
  let right_arr_len = right - mid;

  let mut left_arr = vec![0; left_arr_len];
  let mut right_arr = vec![0; right_arr_len];

  for i in 0..left_arr_len {
    left_arr[i] = arr[left + i];
  }
  for j in 0..right_arr_len {
    right_arr[j] = arr[mid + 1 + j];
  }

  let mut l_idx = 0;
  let mut r_idx = 0;
  let mut idx = left;

  while l_idx < left_arr_len && r_idx < right_arr_len {
    if left_arr[l_idx] <= right_arr[r_idx] {
      arr[idx] = left_arr[l_idx];
      l_idx += 1;
    } else {
      arr[idx] = right_arr[r_idx];
      r_idx += 1;
    }
    idx += 1;
  }

  while l_idx < left_arr_len {
    arr[idx] = left_arr[l_idx];
    l_idx += 1;
    idx += 1;
  }

  while r_idx < right_arr_len {
    arr[idx] = right_arr[r_idx];
    r_idx += 1;
    idx += 1;
  }
}

fn merge_sort_optz_helper(arr: &mut Vec<i32>, l: usize, r: usize) {
  if l >= r {
    return;
  }

  let mid = l + (r - l) / 2;
  merge_sort_optz_helper(arr, l, mid);
  merge_sort_optz_helper(arr, mid + 1, r);
  merge(arr, l, mid, r);
}

pub fn merge_sort_optz(arr: &mut Vec<i32>) {
  merge_sort_optz_helper(arr, 0, arr.len() - 1)
}

pub fn merge_sort(arr: &mut Vec<i32>) {
  let len = arr.len();
  if len < 2 {
    return;
  }

  //divide arr
  let mid = len / 2;
  let mut arr_left = arr[0..mid].to_vec();
  let mut arr_right = arr[mid..].to_vec();

  merge_sort(&mut arr_left);
  merge_sort(&mut arr_right);

  arr.clear();
  arr.extend(merge_two_sorted_array(&arr_left, &arr_right));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_merge_sort() {
    let mut arr = vec![3, 2, 1, 5, 4];
    merge_sort_optz(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
  }
}
