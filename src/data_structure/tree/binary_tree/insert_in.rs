use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::data_structure::queue::Queue;

use super::node::BinaryTreeNode;

pub fn insert_in_binary_tree<T>(
  root: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
  data: T,
) -> Option<Rc<RefCell<BinaryTreeNode<T>>>>
where
  T: Debug + Clone,
{
  let retval = root.clone();
  let new_node = Rc::new(RefCell::new(BinaryTreeNode::new(data)));
  match root {
    Some(root) => {
      let mut queue = Queue::new(100);
      let _ = queue.enqueue(root);
      while let Some(node) = queue.dequeue() {
        if let Some(left) = node.borrow().left.clone() {
          let _ = queue.enqueue(left);
        } else {
          node.borrow_mut().clone().append_left(Some(new_node.clone()));
          break;
        }

        if let Some(right) = node.borrow().right.clone() {
          let _ = queue.enqueue(right);
        } else {
          node.borrow_mut().clone().append_right(Some(new_node.clone()));
          break;
        }
      }

      retval
    }
    None => Some(new_node),
  }
}
