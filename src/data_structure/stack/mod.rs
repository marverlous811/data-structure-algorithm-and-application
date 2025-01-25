pub struct Stack<T>
where
  T: Clone,
{
  data: Vec<T>,
  idx: i32,
  capacity: i32,
}

impl<T> Stack<T>
where
  T: Clone,
{
  pub fn new(cap: usize) -> Self {
    Stack {
      data: Vec::with_capacity(cap),
      idx: -1,
      capacity: cap as i32,
    }
  }

  pub fn push(&mut self, data: T) {
    if self.idx >= self.capacity {
      self.data.reserve_exact(self.capacity as usize);
      self.capacity *= 2;
    }

    self.data.push(data);
    self.idx += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.idx == -1 {
      return None;
    }

    self.idx -= 1;
    self.data.pop()
  }

  pub fn is_empty(&self) -> bool {
    self.idx == 0
  }

  pub fn top(&self) -> Option<&T> {
    if self.idx < 0 {
      return None;
    }

    Some(&self.data[self.idx as usize])
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }

  pub fn capacity(&self) -> usize {
    self.capacity as usize
  }
}

impl<T> Stack<T>
where
  T: Clone,
{
  pub fn from_vec(arr: &Vec<T>) -> Self {
    let mut stack = Self::new(arr.len());
    arr.iter().for_each(|d| stack.push(d.clone()));
    stack
  }

  pub fn flush_to_vec(&mut self) -> Vec<T> {
    let mut result = vec![];
    while let Some(data) = self.pop() {
      result.push(data);
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stack() {
    let mut stack = Stack::new(2);
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
  }

  #[test]
  fn test_stack_by_array() {
    let data = vec![1, 2, 3, 4, 5];
    let mut stack = Stack::new(data.len());
    for num in data.clone() {
      stack.push(num);
    }

    let mut result = vec![];
    while let Some(num) = stack.pop() {
      result.push(num);
    }

    assert_eq!(result, vec![5, 4, 3, 2, 1]);
  }
}
