use rand::Rng;

pub fn random_num(from: i32, to: i32) -> i32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(from..=to)
}

pub fn random_vec(len: usize, from: i32, to: i32) -> Vec<i32> {
  (0..len).map(|_| random_num(from, to)).collect()
}
