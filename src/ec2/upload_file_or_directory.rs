use crate::inputs::input::input;

use std::path::Path;
use std::process;

pub fn upload_file_or_directory(platform: String) {
    let mut path = String::new();
    input("What is the path to your file/directiry?", &mut path);
    path = path.trim().to_string();
    
    let file = is_file(&path);
    let loc = if platform.trim() == "mac" { String::from("~/data/school") } else { String::from("~/data/pc") };

    let mut sure = String::new();
    input(&format!("{}{}", "Are you sure you want to upload to ", if platform.trim() == "mac" { "SCHOOL (y/n)" } else { "PC (y/n)" }), &mut sure);

    if sure.trim() != "y" {
        println!("Okay, Exiting");
        process::exit(1);
    }

    let output;
    if file {
        output = process::Command::new("rsync").args(&["-avL", "--progress", "-e", "ssh -i ~/devdir/ec2/ibrahimfadel.pem", &path, &format!("{}{}", "ec2-user@ec2-52-23-186-153.compute-1.amazonaws.com:", loc)]).output().expect("Failed to upload");
    } else {
        output = process::Command::new("rsync").args(&["-avL", "--progress", "--exclude", "node_modules", "-e", "ssh -i ~/devdir/ec2/ibrahimfadel.pem", &path, &format!("{}{}", "ec2-user@ec2-52-23-186-153.compute-1.amazonaws.com:", loc)]).output().expect("Failed to upload");
    }

    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
}

fn is_file(path: &String) -> bool {
    let file = Path::new(path).is_file();
    file
}