use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::binary_tree::BinaryTreeNode;

pub fn is_tree_equal<T>(
  root1: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
  root2: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
) -> bool
where
  T: Debug + Clone + Eq,
{
  if root1.is_none() && root2.is_none() {
    return true;
  }
  if root1.is_none() || root2.is_none() {
    return false;
  }

  let root1 = root1.unwrap();
  let root2 = root2.unwrap();

  root1.borrow().data == root2.borrow().data
    && is_tree_equal(root1.borrow().left.clone(), root2.borrow().left.clone())
    && is_tree_equal(root1.borrow().right.clone(), root2.borrow().right.clone())
}

#[cfg(test)]
mod test {
  use crate::data_structure::tree::{binary_tree::array_to_bst, equal_tree::is_tree_equal};

  #[test]
  pub fn test_equal_tree() {
    let arr = vec![1, 2, 3, 4, 5];
    let len = arr.len();
    let tree1 = array_to_bst(arr.clone(), 0, len);
    let tree2 = array_to_bst(arr.clone(), 0, len);

    assert_eq!(is_tree_equal(tree1, tree2), true);
  }

  #[test]
  pub fn test_none_tree() {
    let arr = vec![1, 2, 3, 4, 5];
    let len = arr.len();
    let tree1 = array_to_bst(arr.clone(), 0, len);
    assert_eq!(is_tree_equal::<i32>(None, None), true);
    assert_eq!(is_tree_equal(tree1, None), false);
  }

  #[test]
  pub fn test_none_equal_tree() {
    let tree1 = array_to_bst(vec![1, 2, 3, 4, 5], 0, 5);
    let tree2 = array_to_bst(vec![1, 2, 3], 0, 3);

    assert_eq!(is_tree_equal(tree1, tree2), false);
  }
}
