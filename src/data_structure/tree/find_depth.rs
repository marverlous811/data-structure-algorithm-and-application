pub fn find_depth_tree(parents: Vec<i32>) -> i32 {
  let mut max_depth = -1;
  for i in 0..parents.len() {
    let mut current_depth = 0;
    let mut j = i as i32;
    while parents[j as usize] != -1 {
      current_depth += 1;
      j = parents[j as usize];
    }

    if current_depth > max_depth {
      max_depth = current_depth
    }
  }

  max_depth
}

#[cfg(test)]
mod test {
  use crate::data_structure::tree::find_depth::find_depth_tree;

  #[test]
  pub fn test_find_depth_tree() {
    assert_eq!(find_depth_tree(vec![-1, 0, 1, 6, 6, 0, 0, 2, 7]), 4);
  }
}
