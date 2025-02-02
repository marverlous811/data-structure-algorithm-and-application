use std::{cell::RefCell, rc::Rc};

use crate::data_structure::queue::Queue;

use super::node::BinaryTreeNode;

pub fn find_max_node(root: Option<Rc<RefCell<BinaryTreeNode<i32>>>>) -> i32 {
  let mut max_val = i32::MIN;
  if let Some(root) = root {
    let max_left = find_max_node(root.borrow().left.clone());
    let max_right = find_max_node(root.borrow().right.clone());
    let node_val = root.borrow().data;
    max_val = max_val.max(node_val.max(max_left.max(max_right)));
  }

  max_val
}

pub fn find_max_using_level_order(root: Option<Rc<RefCell<BinaryTreeNode<i32>>>>) -> i32 {
  let mut max_val = i32::MIN;
  let mut queue = Queue::new(10);
  if let Some(root) = root {
    let _ = queue.enqueue(root);
  }

  while let Some(node) = queue.dequeue() {
    if max_val < node.borrow().data {
      max_val = node.borrow().data;
    }

    if let Some(left) = node.borrow().left.clone() {
      let _ = queue.enqueue(left);
    }

    if let Some(right) = node.borrow().right.clone() {
      let _ = queue.enqueue(right);
    }
  }

  max_val
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::data_structure::tree::binary_tree::{find_max::find_max_using_level_order, node::BinaryTreeNode};

  use super::find_max_node;

  #[test]
  fn test_max_value_tree() {
    let tree = Rc::new(RefCell::new(BinaryTreeNode::new(1).with_left(4).with_right(2)));

    assert_eq!(find_max_node(Some(tree.clone())), 4);
    assert_eq!(find_max_using_level_order(Some(tree)), 4);
  }
}
