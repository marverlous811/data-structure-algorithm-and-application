use std::{cell::RefCell, rc::Rc};

use crate::data_structure::queue::Queue;

use super::node::{BinaryTreeNode, TreeNode};

pub fn find_level_with_max_sum(root: Option<Rc<RefCell<BinaryTreeNode<i32>>>>) -> usize {
  match root {
    Some(node) => {
      let mut level = 0;
      let mut max_level = 0;
      let mut max_sum = 0;
      let mut current_sum = 0;
      let mut queue = Queue::new(100);

      let _ = queue.enqueue(TreeNode::Node(node));
      let _ = queue.enqueue(TreeNode::Null);
      while let Some(node) = queue.dequeue() {
        match node {
          TreeNode::Node(node) => {
            current_sum += node.borrow().data;
            if let Some(left) = node.borrow().left.clone() {
              let _ = queue.enqueue(TreeNode::Node(left));
            }
            if let Some(right) = node.borrow().right.clone() {
              let _ = queue.enqueue(TreeNode::Node(right));
            }
          }
          TreeNode::Null => {
            if current_sum > max_sum {
              max_sum = current_sum;
              max_level = level;
            }
            current_sum = 0;
            if !queue.is_empty() {
              let _ = queue.enqueue(TreeNode::Null);
            }
            level += 1;
          }
        }
      }

      max_level
    }
    None => 0,
  }
}

#[cfg(test)]
mod test {
  use crate::data_structure::tree::binary_tree::{find_level_max_sum::find_level_with_max_sum, node::array_to_bst};

  #[test]
  pub fn test_find_level_with_max_sum() {
    let arr = vec![1, 2, 3, 4, 5];
    let tree = array_to_bst(arr.clone(), 0, arr.len());
    assert_eq!(find_level_with_max_sum(tree), 1);
  }
}
