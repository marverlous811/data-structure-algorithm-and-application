use std::{cell::RefCell, fmt::Debug, ptr, rc::Rc};

pub struct ThreadedBinaryTreeNode<T> {
  data: T,
  ltag: usize,
  rtag: usize,
  left: Rc<RefCell<ThreadedBinaryTreeNode<T>>>,
  right: Rc<RefCell<ThreadedBinaryTreeNode<T>>>,
}

pub fn inorder_successor<T>(root: Rc<RefCell<ThreadedBinaryTreeNode<T>>>) -> Rc<RefCell<ThreadedBinaryTreeNode<T>>> {
  if root.borrow().rtag == 0 {
    return root.borrow().right.clone();
  }

  let mut pos = root.borrow().right.clone();
  while pos.borrow().rtag == 1 {
    let left = pos.borrow().left.clone();
    pos = left;
  }

  pos
}

pub fn inorder_print<T>(root: Rc<RefCell<ThreadedBinaryTreeNode<T>>>)
where
  T: Debug,
{
  let mut p = inorder_successor(root.clone());
  while !ptr::eq(p.as_ref(), root.as_ref()) {
    p = inorder_successor(p.clone());
    println!("{:?}", p.borrow().data);
  }
}
