use std::{cell::RefCell, rc::Rc};

use super::binary_tree::BinaryTreeNode;

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

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::data_structure::tree::binary_tree::BinaryTreeNode;

  use super::find_max_node;

  #[test]
  fn test_max_value_tree() {
    let tree = Rc::new(RefCell::new(BinaryTreeNode::new(1).with_left(4).with_right(2)));

    assert_eq!(find_max_node(Some(tree)), 4);
  }
}
