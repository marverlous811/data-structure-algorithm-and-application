pub struct Stack<T> {
  data: Vec<T>,
  idx: i32,
  capacity: i32,
}

impl<T> Stack<T> {
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
    if self.idx == 0 {
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
}
