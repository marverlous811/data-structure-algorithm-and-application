use crate::data_structure::{queue::Queue, Stack};

pub fn reserve_order_k_queue(queue: &mut Queue<i32>, k: usize) {
  if queue.is_empty() || queue.size() < k {
    return;
  }

  let mut stack = Stack::new(k);
  for _ in 0..k {
    if let Some(data) = queue.dequeue() {
      stack.push(data);
    }
  }

  while let Some(data) = stack.pop() {
    let _ = queue.enqueue(data);
  }

  for _ in 0..queue.size() - k {
    if let Some(data) = queue.dequeue() {
      let _ = queue.enqueue(data);
    }
  }
}

#[cfg(test)]
mod test {
  use crate::data_structure::queue::Queue;

  use super::reserve_order_k_queue;

  #[test]
  fn test_reserve_k_queue() {
    let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
    let mut queue = Queue::from_vec(&data);

    reserve_order_k_queue(&mut queue, 4);
    let result = queue.to_vec();
    assert_eq!(result, vec![40, 30, 20, 10, 50, 60, 70, 80, 90])
  }
}
