use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::node::BinaryTreeNode;

pub fn mirror_tree<T>(root: &mut Option<Rc<RefCell<BinaryTreeNode<T>>>>)
where
  T: Clone + Debug,
{
  if let Some(node) = root {
    let mut node = node.borrow_mut();
    mirror_tree(&mut node.left.clone());
    mirror_tree(&mut node.right.clone());

    let tmp = node.left.clone();
    node.left = node.right.clone();
    node.right = tmp;
  }
}

#[cfg(test)]
mod test {
  use crate::data_structure::tree::binary_tree::node::BinaryTreeNode;

  use super::mirror_tree;

  #[test]
  fn test_mirror_tree() {
    let data = vec![1, 2, 3, 4, 5];
    let root = BinaryTreeNode::from_sorted_array(&data);
    mirror_tree(&mut Some(root.clone()));

    let result = root.borrow().to_inorder_vec();
    assert_eq!(result, vec![5, 4, 3, 2, 1]);
  }
}
