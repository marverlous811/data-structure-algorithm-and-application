use algorithm_and_application::{random_vec, sort::sort_race};

fn main() {
  let sizes = [10, 100, 1000, 10000, 100000];
  for size in sizes.iter() {
    let input = random_vec(*size, 0, 10000000);
    println!("sorting a vector of size {}", size);
    sort_race(input);
  }
}
