use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::data_structure::queue::Queue;

use super::binary_tree::BinaryTreeNode;

pub fn tree_depth<T>(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> usize
where
  T: Debug + Clone,
{
  match root {
    Some(node) => {
      let left_depth = tree_depth(node.borrow().left.clone());
      let right_depth = tree_depth(node.borrow().right.clone());
      if left_depth > right_depth {
        left_depth + 1
      } else {
        right_depth + 1
      }
    }
    None => 0,
  }
}

#[derive(Clone)]
enum TreeNode<T>
where
  T: Debug + Clone,
{
  Null,
  Node(Rc<RefCell<BinaryTreeNode<T>>>),
}

pub fn tree_depth_without_recusive<T>(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> usize
where
  T: Debug + Clone,
{
  let mut retval = 0;
  if let Some(node) = root {
    let mut queue = Queue::new(100);
    let _ = queue.enqueue(TreeNode::Node(node));
    let _ = queue.enqueue(TreeNode::Null);

    while let Some(tmp) = queue.dequeue() {
      match tmp {
        TreeNode::Null => {
          if !queue.is_empty() {
            let _ = queue.enqueue(TreeNode::Null);
          }
          retval += 1;
        }
        TreeNode::Node(node) => {
          if let Some(left) = node.borrow().left.clone() {
            let _ = queue.enqueue(TreeNode::Node(left));
          }
          if let Some(right) = node.borrow().right.clone() {
            let _ = queue.enqueue(TreeNode::Node(right));
          }
        }
      }
    }
  }

  retval
}

#[cfg(test)]
mod test {
  use crate::data_structure::tree::{
    binary_tree::array_to_bst,
    tree_depth::{tree_depth, tree_depth_without_recusive},
  };

  #[test]
  pub fn test_tree_depth() {
    let arr = vec![1, 2, 3, 4, 5];
    let root = array_to_bst(arr.clone(), 0, arr.len());
    assert_eq!(tree_depth(root.clone()), 3);
    assert_eq!(tree_depth_without_recusive(root.clone()), 3);
  }
}
