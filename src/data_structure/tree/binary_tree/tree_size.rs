use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::node::BinaryTreeNode;

pub fn binary_tree_size<T>(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> usize
where
  T: Debug + Clone,
{
  match root {
    Some(node) => binary_tree_size(node.borrow().left.clone()) + 1 + binary_tree_size(node.borrow().right.clone()),
    None => 0,
  }
}

#[cfg(test)]
mod test {
  use crate::data_structure::tree::binary_tree::node::array_to_bst;

  use super::binary_tree_size;

  #[test]
  fn test_binary_tree_size() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let root = array_to_bst(arr.clone(), 0, arr.len());
    assert_eq!(binary_tree_size(root), 9);
  }
}
