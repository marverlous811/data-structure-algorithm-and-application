use std::{env, path::Path};

use algorithm_and_application::read_file_to_vec;

fn main() {
  let dir = env::current_dir().unwrap();
  let file_a = Path::new(&dir).join("examples").join("text_diff").join("a.txt");
  let file_b = Path::new(&dir).join("examples").join("text_diff").join("b.txt");

  let a = read_file_to_vec(file_a.to_str().unwrap());
  let b = read_file_to_vec(file_b.to_str().unwrap());

  algorithm_and_application::text_diff_print(&a, &b);
}
