use std::{fs::File, io::BufRead};

pub fn read_file_to_vec(filename: &str) -> Vec<String> {
  let file = File::open(filename);
  match file {
    Ok(f) => {
      let reader = std::io::BufReader::new(f);
      let lines = reader.lines().map(|l| l.expect("Could not parser Line")).collect();
      lines
    }
    Err(_) => vec![],
  }
}
