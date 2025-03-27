use std::{cell::RefCell, rc::Rc};

pub trait IGenericTreeNode {
  type Payload;

  fn get(&self) -> Self::Payload;
}

pub struct GenericTreeNode<T> {
  data: T,
  pub first_child: Option<Rc<RefCell<GenericTreeNode<T>>>>,
  pub next_sibling: Option<Rc<RefCell<GenericTreeNode<T>>>>,
}

impl<T> IGenericTreeNode for GenericTreeNode<T>
where
  T: Eq + Clone,
{
  type Payload = T;
  fn get(&self) -> Self::Payload {
    self.data.clone()
  }
}
