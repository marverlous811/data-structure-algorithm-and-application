use crate::data_structure::{queue::Queue, Stack};

pub fn interleaving_queue(queue: &mut Queue<i32>) {
  if queue.size() % 2 != 0 {
    return;
  }

  let mut stack = Stack::new(queue.size());
  let pivot_point = queue.size() / 2;
  for _ in 0..pivot_point {
    if let Some(data) = queue.dequeue() {
      stack.push(data);
    }
  }

  while let Some(data) = stack.pop() {
    let _ = queue.enqueue(data);
  }

  for _ in 0..pivot_point {
    if let Some(data) = queue.dequeue() {
      let _ = queue.enqueue(data);
    }
  }

  for _ in 0..pivot_point {
    if let Some(data) = queue.dequeue() {
      stack.push(data);
    }
  }

  while let Some(data) = stack.pop() {
    let _ = queue.enqueue(data);
    if let Some(other) = queue.dequeue() {
      let _ = queue.enqueue(other);
    }
  }
}

#[cfg(test)]
mod test {
  use crate::data_structure::queue::Queue;

  use super::interleaving_queue;

  #[test]
  fn test_interleaving_queue() {
    let data = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut queue = Queue::from_vec(&data);
    interleaving_queue(&mut queue);

    let result = queue.to_vec();
    assert_eq!(result, vec![11, 16, 12, 17, 13, 18, 14, 19, 15, 20]);
  }
}
