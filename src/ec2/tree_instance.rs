use std::process::Command;

pub fn tree_instance() {
  let output = Command::new("ls").arg("-la").output().expect("failed to tree EC2 instance");
  println!("{}", String::from_utf8_lossy(&output.stdout));
}