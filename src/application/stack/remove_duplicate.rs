struct Idx(Option<usize>);

impl Idx {
  fn new() -> Self {
    Idx(None)
  }

  fn is_empty(&self) -> bool {
    match self.0 {
      Some(_) => false,
      None => true,
    }
  }

  fn get(&self) -> usize {
    match self.0 {
      Some(i) => i,
      None => 0,
    }
  }

  fn increase(&mut self) {
    match self.0 {
      Some(i) => self.0 = Some(i + 1),
      None => self.0 = Some(0),
    }
  }

  fn decrease(&mut self) {
    match self.0 {
      Some(i) => {
        if i == 0 {
          self.0 = None;
        } else {
          self.0 = Some(i - 1)
        }
      }
      None => self.0 = Some(0),
    }
  }
}

pub fn remove_duplicate_char(data: &str) -> String {
  let mut result = data.as_bytes().to_vec();
  let mut ptr = Idx::new();
  let mut i = 0;
  let len = data.len();
  while i < len {
    if ptr.is_empty() || result[ptr.get()] != result[i] {
      ptr.increase();
      let idx = ptr.get();
      result[idx] = result[i];
      i += 1;
    } else {
      while i < len && result[ptr.get()] == result[i] {
        i += 1;
      }
      ptr.decrease();
    }
  }

  ptr.increase();
  let idx = ptr.get();
  result.truncate(idx);
  String::from_utf8_lossy(&result).to_string()
}

#[cfg(test)]
mod test {
  #[test]
  fn test_remove_duplicate_char() {
    assert_eq!(super::remove_duplicate_char("mississippi"), "m");
    assert_eq!(super::remove_duplicate_char("careermonk"), "camonk");
  }
}
