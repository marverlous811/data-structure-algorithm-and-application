use super::Stack;

pub struct Queue<T>
where
  T: Clone,
{
  data: Vec<Option<T>>,
  capacity: usize,
  front: i32,
  rear: i32,
  size: usize,
}

impl<T> Queue<T>
where
  T: Clone,
{
  pub fn new(capacity: usize) -> Self {
    let data = vec![None; capacity];

    Self {
      data,
      capacity,
      front: -1,
      rear: -1,
      size: 0,
    }
  }

  pub fn is_full(&self) -> bool {
    self.size == self.capacity
  }

  pub fn is_empty(&self) -> bool {
    self.front == -1
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn enqueue(&mut self, data: T) -> anyhow::Result<()> {
    if self.is_full() {
      anyhow::bail!("Queue is full");
    }

    self.rear = (self.rear + 1) % self.capacity as i32;
    self.data[self.rear as usize] = Some(data);

    if self.front == -1 {
      self.front = self.rear
    }
    self.size += 1;

    Ok(())
  }

  pub fn dequeue(&mut self) -> Option<T> {
    if self.is_empty() {
      return None;
    }

    let data = self.data[self.front as usize].clone();

    if self.front == self.rear {
      self.front = -1;
      self.rear = -1
    } else {
      self.front = (self.front + 1) % self.capacity as i32
    }
    self.size -= 1;

    data
  }

  pub fn resever(&mut self) -> anyhow::Result<()> {
    let mut stack = Stack::<T>::new(self.capacity);

    while let Some(data) = self.dequeue() {
      stack.push(data);
    }

    while let Some(data) = stack.pop() {
      self.enqueue(data)?;
    }

    Ok(())
  }
}

impl<T> Queue<T>
where
  T: Clone,
{
  pub fn from_vec(arr: &Vec<T>) -> Self {
    let mut queue = Queue::new(arr.len());
    for data in arr {
      let _ = queue.enqueue(data.clone());
    }

    queue
  }

  pub fn to_vec(&mut self) -> Vec<T> {
    let mut result = vec![];
    while let Some(data) = self.dequeue() {
      result.push(data);
    }

    result
  }
}

#[cfg(test)]
mod test {
  use super::Queue;

  #[test]
  pub fn test_queue() {
    let mut queue = Queue::<i32>::new(5);

    assert_eq!(queue.is_empty(), true);
    let res = queue.enqueue(1);
    assert!(res.is_ok());
    assert_eq!(queue.size(), 1);

    assert_eq!(queue.is_empty(), false);
    let current = queue.dequeue();

    assert_eq!(current, Some(1));
    assert_eq!(queue.is_empty(), true);
    assert_eq!(queue.size(), 0);
  }

  #[test]
  pub fn test_full_queue() {
    let mut queue = Queue::<i32>::new(5);

    let _ = queue.enqueue(1);
    let _ = queue.enqueue(2);
    let _ = queue.enqueue(3);
    let _ = queue.enqueue(4);
    let _ = queue.enqueue(5);

    assert_eq!(queue.size(), 5);
    assert_eq!(queue.is_full(), true);
  }

  #[test]
  pub fn test_reserve_queue() {
    let data = vec![1, 2, 3, 4, 5];
    let mut queue = Queue::<i32>::new(data.len());
    for num in data {
      let _ = queue.enqueue(num);
    }

    let _ = queue.resever();
    let mut result = vec![];
    while let Some(data) = queue.dequeue() {
      result.push(data);
    }

    assert_eq!(result, vec![5, 4, 3, 2, 1]);
  }
}
