use algorithm_and_application::{
  algorithms::sort::{sort_race, SortKind},
  helper::random_vec,
};

fn main() {
  let sizes = [10, 100, 1_000, 10_000, 100_000, 1_000_000];
  let kinds = vec![SortKind::MergeSortOptz, SortKind::QuickSort, SortKind::MergeSort];
  for size in sizes.iter() {
    let input = random_vec(*size, 0, 10_000_000);
    println!("sorting a vector of size {}", size);
    sort_race(&input, &kinds);
  }
}
