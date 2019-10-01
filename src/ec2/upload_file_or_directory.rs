use std::path::Path;
use std::process;
use std::process::Command;

use crate::inputs::input::input;

pub fn upload_file_or_directory(platform: String) {
    let mut path = String::new();
    input("What is the path to your file/directory?", &mut path);
    path = path.trim().to_string();
    let file: bool = is_file(&path);

    if file {
        upload_file(&path, &platform);
    } else if !file {
        upload_directory(&path);
    } else {
        println!("Error, invalid path. Not a file or directory!");
        process::exit(1);
    }
}

fn upload_file(path: &String, platform: &String) {
    let output;
    if platform.trim().to_lowercase() == "mac" {
        let mut sure = String::new();
        input("Are you sure you want to upload to SCHOOL? (y/n)", &mut sure);
        if sure.trim() == "y" {
            output = Command::new("rsync").args(&["-avL", "--progress", "-e", "ssh -i ~/devdir/ec2/ibrahimfadel.pem", "../rorkejs/src/rorke.js", "ec2-user@ec2-52-23-186-153.compute-1.amazonaws.com:~/data/school"]).output().expect("BAD");  
        } else {
            println!("Okay, exiting...");
            process::exit(1);
        }
    } else {
        let mut sure = String::new();
        input("Are you sure you want to upload to PC? (y/n)", &mut sure);
        if sure.trim() == "y" {
            output = Command::new("rsync").args(&["-avL", "--progress", "-e", "ssh -i ~/devdir/ec2/ibrahimfadel.pem", "../rorkejs/src/rorke.js", "ec2-user@ec2-52-23-186-153.compute-1.amazonaws.com:~/data/pc"]).output().expect("BAD");
        } else {
            println!("Okay, exiting...");
            process::exit(1);
        }
    }
    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }

  println!("{}", String::from_utf8_lossy(&output.stdout));

}

fn upload_directory(path: &String) {

}

fn is_file(path: &String) -> bool {
    let file = Path::new(path).is_file();
    file
}