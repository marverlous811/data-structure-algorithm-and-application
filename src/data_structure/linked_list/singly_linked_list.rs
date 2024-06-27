use std::{cell::RefCell, rc::Rc};

pub struct SinglyNode {
  pub val: i32,
  pub next: Option<Rc<RefCell<SinglyNode>>>,
}

impl SinglyNode {
  pub fn new(val: i32) -> Self {
    Self { val, next: None }
  }
}

pub struct SinglyLinkedList {
  root: Option<Rc<RefCell<SinglyNode>>>,
}

impl SinglyLinkedList {
  pub fn default() -> Self {
    Self { root: None }
  }

  pub fn from_arr(arr: &Vec<i32>) -> Self {
    let mut res = Self::default();
    for (_, i) in arr.iter().enumerate() {
      res.push(*i);
    }
    res
  }

  fn traversing_mut<F>(&self, f: &mut F)
  where
    F: FnMut(i32),
  {
    let mut cur = self.root.clone();
    while let Some(node) = cur {
      f((*node).borrow().val);
      cur = (*node).borrow().next.clone();
    }
  }

  pub fn len(&self) -> usize {
    let mut i = 0;
    let mut cur = self.root.clone();
    while let Some(node) = cur {
      i += 1;
      cur = (*node).borrow().next.clone();
    }
    i
  }

  pub fn to_vec(&self) -> Vec<i32> {
    let arr = RefCell::new(vec![]);
    let mut arr_mut = arr.borrow_mut();

    self.traversing_mut(
      &mut (move |data: i32| {
        arr_mut.push(data);
      }),
    );

    let res = arr.borrow().iter().map(|d| *d).collect();
    res
  }

  pub fn push(&mut self, data: i32) {
    let new_node = Some(Rc::new(RefCell::new(SinglyNode::new(data))));
    match self.root.clone() {
      Some(node) => {
        let mut cur = Some(node.clone());
        let mut prev = Some(node);
        while let Some(node) = cur.clone() {
          prev = cur;
          cur = (*node).borrow().next.clone()
        }

        if let Some(node) = prev {
          (*node).borrow_mut().next = new_node;
        }
      }
      None => {
        self.root = new_node;
      }
    };
  }

  pub fn insert_at(&mut self, data: i32, pos: usize) {
    let mut new_node = SinglyNode::new(data);
    let mut idx = 0;
    let mut cur = self.root.clone();
    let mut prev: Option<Rc<RefCell<SinglyNode>>> = None;
    while let Some(node) = cur.clone() {
      if idx == pos {
        new_node.next = cur;
        if let Some(prev_node) = prev {
          (*prev_node).borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
        } else {
          self.root = Some(Rc::new(RefCell::new(new_node)));
        }

        return;
      }
      idx += 1;
      prev = cur;
      cur = (*node).borrow().next.clone();
    }

    if let Some(prev_node) = prev {
      (*prev_node).borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
    }
  }

  pub fn delete_at(&mut self, pos: usize) -> Option<i32> {
    let mut idx = 0;
    let mut cur = self.root.clone();
    let mut prev: Option<Rc<RefCell<SinglyNode>>> = None;
    while let Some(node) = cur.clone() {
      if idx == pos {
        if let Some(prev_node) = prev {
          (*prev_node).borrow_mut().next = (*node).borrow_mut().next.clone();
        } else {
          self.root = (*node).borrow_mut().next.clone();
        }
        let res = (*node).borrow().val;
        return Some(res);
      }
      idx += 1;
      prev = cur;
      cur = (*node).borrow().next.clone();
    }

    None
  }
}

#[cfg(test)]
mod test {
  use crate::SinglyLinkedList;

  #[test]
  fn simple_test() {
    let expected = vec![1, 2, 3];
    let linked_list = SinglyLinkedList::from_arr(&expected);

    let arr = linked_list.to_vec();
    assert_eq!(linked_list.len(), expected.len());
    assert_eq!(arr, expected);
  }

  #[test]
  fn insert_at_first_test() {
    let input = vec![1, 2, 3];
    let mut linked_list = SinglyLinkedList::from_arr(&input);

    linked_list.insert_at(5, 0);
    assert_eq!(linked_list.to_vec(), vec![5, 1, 2, 3]);
  }

  #[test]
  fn insert_at_middle_test() {
    let input = vec![1, 2, 3];
    let mut linked_list = SinglyLinkedList::from_arr(&input);

    linked_list.insert_at(5, 2);
    assert_eq!(linked_list.to_vec(), vec![1, 2, 5, 3]);
  }

  #[test]
  fn insert_at_last_test() {
    let input = vec![1, 2, 3];
    let mut linked_list = SinglyLinkedList::from_arr(&input);

    linked_list.insert_at(5, 3);
    assert_eq!(linked_list.to_vec(), vec![1, 2, 3, 5]);
  }

  #[test]
  fn delete_at_first() {
    let mut linked_list = SinglyLinkedList::from_arr(&vec![1, 2, 3]);
    let data = linked_list.delete_at(0);
    assert_eq!(data, Some(1));
    assert_eq!(linked_list.to_vec(), vec![2, 3]);
  }

  #[test]
  fn delete_at_middle() {
    let mut linked_list = SinglyLinkedList::from_arr(&vec![1, 2, 3]);
    let data = linked_list.delete_at(1);
    assert_eq!(data, Some(2));
    assert_eq!(linked_list.to_vec(), vec![1, 3]);
  }

  #[test]
  fn delete_at_last() {
    let mut linked_list = SinglyLinkedList::from_arr(&vec![1, 2, 3]);
    let data = linked_list.delete_at(2);
    assert_eq!(data, Some(3));
    assert_eq!(linked_list.to_vec(), vec![1, 2]);
  }
}
