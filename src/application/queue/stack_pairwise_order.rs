use crate::data_structure::{queue::Queue, Stack};

pub fn check_stack_pairwise_order(stack: &mut Stack<i32>) -> bool {
  let mut queue = Queue::<i32>::new(stack.capacity());
  let mut result = true;
  while let Some(num) = stack.pop() {
    let _ = queue.enqueue(num);
  }
  while let Some(num) = queue.dequeue() {
    let _ = stack.push(num);
  }

  while let Some(data) = stack.pop() {
    let _ = queue.enqueue(data);
    if let Some(next) = stack.pop() {
      let _ = queue.enqueue(next);
      if (data - next).abs() != 1 {
        result = false
      }
    }
  }

  result
}

#[cfg(test)]
mod test {

  use crate::data_structure::Stack;

  use super::check_stack_pairwise_order;

  #[test]
  fn test_stack_pairwise_order() {
    let data = vec![4, 5, -2, -3, 11, 10, 5, 6, 20];
    let mut stack = Stack::from_vec(&data);

    let result = check_stack_pairwise_order(&mut stack);
    assert_eq!(result, true);
  }
}
