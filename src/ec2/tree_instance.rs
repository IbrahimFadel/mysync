use std::process::Command;

pub fn tree_instance() {
  let output = Command::new("ssh").args(&["-i", "~/devdir/ec2/ibrahimfadel.pem", "ec2-user@ec2-52-23-186-153.compute-1.amazonaws.com", "tree"]).output().expect("BAD");

  if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }

  println!("{}", String::from_utf8_lossy(&output.stdout));
}