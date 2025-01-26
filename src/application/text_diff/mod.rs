use crate::algorithms::{lcs_table, reconstruct_elements};

pub fn text_diff_print(file_a: &Vec<String>, file_b: &Vec<String>) {
  let lcs = reconstruct_elements(file_a, file_b, &lcs_table(file_a, file_b));

  let mut line_a = 0;
  let mut line_b = 0;

  for line in lcs {
    while file_a[line_a] != line {
      println!("- {}", file_a[line_a]);
      line_a += 1;
    }
    while file_b[line_b] != line {
      println!("+ {}", file_b[line_b]);
      line_b += 1;
    }

    assert!(file_a[line_a] == file_b[line_b]);
    println!("  {}", line);
    line_a += 1;
    line_b += 1;
  }

  while line_a < file_a.len() {
    println!("- {}", file_a[line_a]);
    line_a += 1;
  }

  while line_b < file_b.len() {
    println!("+ {}", file_b[line_b]);
    line_b += 1;
  }
}
