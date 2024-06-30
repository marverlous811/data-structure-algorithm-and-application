pub fn time_measure<T>(func: T) -> std::time::Duration
where
  T: Fn(),
{
  let start = std::time::Instant::now();
  // code to measure
  func();
  let end = std::time::Instant::now();
  end - start
}

pub fn duration_as_string(duration: std::time::Duration) -> String {
  let micro: u128 = duration.as_micros();
  if micro < 1000 {
    format!("{} μs", micro)
  } else if micro < 1_000_000 {
    format!("{} ms", duration.as_millis())
  } else {
    format!("{} s", duration.as_secs())
  }
}
