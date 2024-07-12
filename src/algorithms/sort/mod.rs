use crate::{duration_as_string, time_measure};

mod bubble_sort;
mod merge_sort;
mod quick_sort;

#[derive(Debug)]
enum SortKind {
  BubbleSort,
  QuickSort,
  MergeSort,
  MergeSortOptz,
}

pub fn sort_race(input: Vec<i32>) {
  let kinds = vec![SortKind::MergeSortOptz, SortKind::QuickSort, SortKind::MergeSort];
  for kind in kinds.iter() {
    let sort_func = sort_func_factory(kind, input.clone());
    let sort_time = time_measure(sort_func);
    println!("{:?}: {}", kind, duration_as_string(sort_time));
  }
}

fn sort_func_factory(kind: &SortKind, input: Vec<i32>) -> Box<dyn Fn()> {
  match kind {
    SortKind::BubbleSort => Box::new(sort_func_builder(input.clone(), bubble_sort::bubble_sort)),
    SortKind::QuickSort => Box::new(sort_func_builder(input.clone(), quick_sort::quick_sort)),
    SortKind::MergeSort => Box::new(sort_func_builder(input.clone(), merge_sort::merge_sort)),
    SortKind::MergeSortOptz => Box::new(sort_func_builder(input.clone(), merge_sort::merge_sort_optz)),
  }
}

pub fn sort_func_builder<F>(input: Vec<i32>, f: F) -> impl Fn()
where
  F: Fn(&mut Vec<i32>),
{
  let closure = move || {
    let mut arr = input.clone();
    f(&mut arr);
  };
  closure
}
