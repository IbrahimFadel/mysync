use std::io::stdin;

pub fn input(msg: &str, v: &mut String) {
  println!("{}", msg);
  stdin().read_line(v).unwrap();
}