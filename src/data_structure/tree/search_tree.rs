use std::{cell::RefCell, rc::Rc};

use crate::data_structure::queue::Queue;

use super::binary_tree::BinaryTreeNode;

pub fn find_in_binary_tree(root: Option<Rc<RefCell<BinaryTreeNode<i32>>>>, data: i32) -> bool {
  if let Some(root) = root {
    if root.borrow().data == data {
      true
    } else {
      let retval = find_in_binary_tree(root.borrow().left.clone(), data);
      if retval {
        retval
      } else {
        find_in_binary_tree(root.borrow().right.clone(), data)
      }
    }
  } else {
    false
  }
}

pub fn find_binary_tree_using_level_order(root: Option<Rc<RefCell<BinaryTreeNode<i32>>>>, data: i32) -> bool {
  if root.is_none() {
    return false;
  }

  let root = root.unwrap();
  let mut queue = Queue::new(100);
  let _ = queue.enqueue(root);

  while let Some(node) = queue.dequeue() {
    if node.borrow().data == data {
      return true;
    }

    if let Some(left) = node.borrow().left.clone() {
      let _ = queue.enqueue(left);
    }

    if let Some(right) = node.borrow().right.clone() {
      let _ = queue.enqueue(right);
    }
  }

  false
}

#[cfg(test)]
mod test {
  use crate::data_structure::tree::{binary_tree::array_to_bst, search_tree::find_binary_tree_using_level_order};

  use super::find_in_binary_tree;

  #[test]
  fn test_find_value_in_tree() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let root = array_to_bst(arr.clone(), 0, arr.len());

    assert_eq!(find_in_binary_tree(root.clone(), 3), true);
    assert_eq!(find_in_binary_tree(root.clone(), 10), false);

    assert_eq!(find_binary_tree_using_level_order(root.clone(), 3), true);
    assert_eq!(find_binary_tree_using_level_order(root.clone(), 10), false);
  }
}
