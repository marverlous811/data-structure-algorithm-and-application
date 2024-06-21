pub fn lcs_table<T>(first: &Vec<T>, second: &Vec<T>) -> Vec<Vec<i32>>
where
  T: Eq,
{
  let mut dp = vec![vec![0; second.len() + 1]; first.len() + 1];
  for i in 1..=first.len() {
    for j in 1..=second.len() {
      if first[i - 1] == second[j - 1] {
        dp[i][j] = 1 + dp[i - 1][j - 1];
      } else {
        dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
      }
    }
  }
  dp
}

pub fn longest_common_subsequence<T>(first: Vec<T>, second: Vec<T>) -> i32
where
  T: Eq,
{
  let dp = lcs_table(&first, &second);
  dp[first.len()][second.len()]
}

pub fn reconstruct_elements<T>(first: &Vec<T>, second: &Vec<T>, dp: &Vec<Vec<i32>>) -> Vec<T>
where
  T: Clone + Eq,
{
  let mut i = first.len();
  let mut j = second.len();
  let mut retval = Vec::new();
  while i > 0 && j > 0 {
    if first[i - 1] == second[j - 1] {
      retval.push(first[i - 1].clone());
      i -= 1;
      j -= 1;
    } else {
      if dp[i - 1][j] > dp[i][j - 1] {
        i -= 1;
      } else {
        j -= 1;
      }
    }
  }

  retval.into_iter().rev().collect()
}

#[cfg(test)]
mod test {
  #[test]
  fn lcs_example_1() {
    let first = vec![1, 4, 5, 6, 9, 10, 11];
    let second = vec![6, 4, 5, 9, 11];
    assert_eq!(super::longest_common_subsequence(first, second), 4);
  }

  #[test]
  fn lcs_example_2() {
    let text1 = "abcde".chars().collect();
    let text2 = "ace".chars().collect();
    assert_eq!(super::longest_common_subsequence(text1, text2), 3);
  }

  #[test]
  fn lcs_example_3() {
    let text1 = "ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".chars().collect();
    let text2 = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".chars().collect();
    assert_eq!(super::longest_common_subsequence(text1, text2), 0);
  }

  #[test]
  fn lcs_restruct_el_example_1() {
    let first = vec![1, 4, 5, 6, 9, 10, 11];
    let second = vec![6, 4, 5, 9, 11];
    let dp = super::lcs_table(&first, &second);
    assert_eq!(super::reconstruct_elements(&first, &second, &dp), vec![4, 5, 9, 11]);
  }
}
