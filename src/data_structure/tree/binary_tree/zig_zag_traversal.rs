use std::{cell::RefCell, fmt::Debug, mem::swap, rc::Rc};

use crate::data_structure::Stack;

use super::node::BinaryTreeNode;

pub fn zig_zag_traversal<T>(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>, arr: &mut Vec<T>)
where
  T: Debug + Clone,
{
  if let Some(node) = root {
    let mut left_to_right = true;
    let mut current_level = Stack::new(node.borrow().size());
    let mut next_level = Stack::new(node.borrow().size());
    current_level.push(node);
    while !current_level.is_empty() {
      let tmp = current_level.pop();
      if let Some(node) = tmp {
        arr.push(node.borrow().data.clone());
        if left_to_right {
          if let Some(left) = node.borrow().left.clone() {
            next_level.push(left);
          }
          if let Some(right) = node.borrow().right.clone() {
            next_level.push(right);
          }
        } else {
          if let Some(right) = node.borrow().right.clone() {
            next_level.push(right);
          }
          if let Some(left) = node.borrow().left.clone() {
            next_level.push(left);
          }
        }
      }

      if current_level.is_empty() {
        left_to_right = !left_to_right;
        swap(&mut current_level, &mut next_level);
      }
    }
  }
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::data_structure::tree::binary_tree::node::BinaryTreeNode;

  #[test]
  fn test_zig_zag_traversal() {
    let tree = Rc::new(RefCell::new(
      BinaryTreeNode::new(1)
        .append_left(Some(Rc::new(RefCell::new(
          BinaryTreeNode::new(2).with_left(4).with_right(5),
        ))))
        .append_right(Some(Rc::new(RefCell::new(
          BinaryTreeNode::new(3).with_left(6).with_right(7),
        )))),
    ));
    let mut result = Vec::new();
    super::zig_zag_traversal(Some(tree), &mut result);

    assert_eq!(result, vec![1, 3, 2, 4, 5, 6, 7]);
  }
}
