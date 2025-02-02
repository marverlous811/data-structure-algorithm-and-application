use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone)]
pub enum TreeNode<T>
where
  T: Debug + Clone,
{
  Null,
  Node(Rc<RefCell<BinaryTreeNode<T>>>),
}

#[derive(Clone)]
pub struct BinaryTreeNode<T>
where
  T: Debug + Clone,
{
  pub data: T,
  pub left: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
  pub right: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
}

impl<T> BinaryTreeNode<T>
where
  T: Debug + Clone,
{
  pub fn new(data: T) -> Self {
    Self {
      data,
      left: None,
      right: None,
    }
  }

  pub fn with_left(mut self, data: T) -> Self {
    let child = BinaryTreeNode::new(data);
    self.left = Some(Rc::new(RefCell::new(child)));
    self
  }

  pub fn with_right(mut self, data: T) -> Self {
    let child = BinaryTreeNode::new(data);
    self.right = Some(Rc::new(RefCell::new(child)));
    self
  }

  pub fn append_left(mut self, node: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> Self {
    self.left = node;
    self
  }

  pub fn append_right(mut self, node: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> Self {
    self.right = node;
    self
  }
}

pub fn inorder_traversal<T, F>(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>, f: &mut F)
where
  T: Debug + Clone,
  F: FnMut(T),
{
  if let Some(node) = root {
    inorder_traversal(node.borrow().left.clone(), f);
    f(node.borrow().data.clone());
    inorder_traversal(node.borrow().right.clone(), f);
  }
}
pub fn array_to_bst<T>(arr: Vec<T>, start: usize, end: usize) -> Option<Rc<RefCell<BinaryTreeNode<T>>>>
where
  T: Debug + Clone,
{
  if start >= end {
    None
  } else {
    let mid = (start + end) / 2;
    let node = BinaryTreeNode::new(arr[mid].clone())
      .append_left(array_to_bst(arr.clone(), start, mid))
      .append_right(array_to_bst(arr.clone(), mid + 1, end));

    Some(Rc::new(RefCell::new(node)))
  }
}

#[cfg(test)]
mod test {
  use super::{array_to_bst, inorder_traversal};

  #[test]
  fn to_inorder_vec() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let tree = array_to_bst(arr.clone(), 0, arr.len());
    let mut result = vec![];
    inorder_traversal(tree, &mut |data: i32| {
      result.push(data);
    });

    assert_eq!(result, arr);
  }
}
