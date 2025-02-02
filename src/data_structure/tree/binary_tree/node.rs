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

impl<T> BinaryTreeNode<T>
where
  T: Debug + Clone,
{
  pub fn from_sorted_array(arr: &Vec<T>) -> Rc<RefCell<BinaryTreeNode<T>>> {
    let root = array_to_bst(arr.clone(), 0, arr.len());
    root.unwrap()
  }

  pub fn to_inorder_vec(&self) -> Vec<T> {
    let mut result = vec![];
    inorder_traversal(Some(Rc::new(RefCell::new(self.clone()))), &mut |data: T| {
      result.push(data);
    });

    result
  }

  pub fn to_pre_order_vec(&self) -> Vec<T> {
    let mut result = vec![];
    preorder_tranversal(Some(Rc::new(RefCell::new(self.clone()))), &mut |data: T| {
      result.push(data);
    });

    result
  }

  pub fn to_post_order_vec(&self) -> Vec<T> {
    let mut result = vec![];
    postorder_tranversal(Some(Rc::new(RefCell::new(self.clone()))), &mut |data: T| {
      result.push(data);
    });

    result
  }

  pub fn size(&self) -> usize {
    let mut size = 0;
    inorder_traversal(Some(Rc::new(RefCell::new(self.clone()))), &mut |_| {
      size += 1;
    });

    size
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

pub fn preorder_tranversal<T, F>(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>, f: &mut F)
where
  T: Debug + Clone,
  F: FnMut(T),
{
  if let Some(node) = root {
    f(node.borrow().data.clone());
    preorder_tranversal(node.borrow().left.clone(), f);
    preorder_tranversal(node.borrow().right.clone(), f);
  }
}

pub fn postorder_tranversal<T, F>(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>, f: &mut F)
where
  T: Debug + Clone,
  F: FnMut(T),
{
  if let Some(node) = root {
    postorder_tranversal(node.borrow().left.clone(), f);
    postorder_tranversal(node.borrow().right.clone(), f);
    f(node.borrow().data.clone());
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

pub struct TreeBuilder<T>
where
  T: Debug + Clone + Eq,
{
  inorder: Vec<T>,
  preorder: Vec<T>,
  preorder_idx: usize,
}

impl<T> TreeBuilder<T>
where
  T: Debug + Clone + Eq,
{
  pub fn new(inorder: Vec<T>, preorder: Vec<T>) -> Self {
    Self {
      inorder,
      preorder,
      preorder_idx: 0,
    }
  }

  pub fn build(&mut self) -> Option<Rc<RefCell<BinaryTreeNode<T>>>> {
    self.build_tree(0, self.inorder.len() - 1)
  }

  fn build_tree(&mut self, inorder_start: usize, inorder_end: usize) -> Option<Rc<RefCell<BinaryTreeNode<T>>>> {
    if inorder_start > inorder_end {
      return None;
    }

    let data = self.preorder[self.preorder_idx].clone();
    let new_node = Rc::new(RefCell::new(BinaryTreeNode::new(data.clone())));
    self.preorder_idx += 1;

    if inorder_start == inorder_end {
      return Some(new_node);
    }

    let inorder_idx = || -> usize {
      for i in inorder_start..=inorder_end {
        if self.inorder[i] == data {
          return i as usize;
        }
      }
      panic!("Invalid inorder index");
    }();

    new_node.borrow_mut().left = self.build_tree(inorder_start, inorder_idx - 1);
    new_node.borrow_mut().right = self.build_tree(inorder_idx + 1, inorder_end);

    Some(new_node)
  }
}

#[cfg(test)]
mod test {
  use super::array_to_bst;

  #[test]
  fn test_to_inorder_vec() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let tree = array_to_bst(arr.clone(), 0, arr.len());
    let result = tree.unwrap().borrow().to_inorder_vec();

    assert_eq!(result, arr);
  }

  #[test]
  fn test_size() {
    let arr = vec![1, 2, 3, 4, 5];
    let tree = array_to_bst(arr.clone(), 0, arr.len());
    let result = tree.unwrap().borrow().size();
    assert_eq!(result, arr.len());
  }

  #[test]
  fn test_build_tree() {
    let preorder = vec![1, 2, 3, 4, 5, 6];
    let inorder = vec![3, 2, 4, 1, 6, 5];
    let mut builder = super::TreeBuilder::new(inorder.clone(), preorder.clone());
    let tree = builder.build();

    assert!(tree.is_some());

    let tree = tree.unwrap();
    let preorder_result = tree.borrow().to_pre_order_vec();
    assert_eq!(preorder_result, preorder);

    let inorder_result = tree.borrow().to_inorder_vec();
    assert_eq!(inorder_result, inorder);

    let postorder_result = tree.borrow().to_post_order_vec();
    assert_eq!(postorder_result, vec![3, 4, 2, 6, 5, 1]);
  }
}
