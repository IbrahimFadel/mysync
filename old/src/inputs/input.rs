use std::io::stdin;

pub fn input(msg: &str, v: &mut String) {
    println!("{}", msg);

    stdin().read_line(v).unwrap();



    // "Hello"
    // return "Hello";
    // let hello_world: &'static str = "Hello, world!";
    // return hello_world;
}

// use std::io::{self, Read};

// pub fn input() -> io::Result<()> {
//     let mut buffer = String::new();
//     let stdin = io::stdin();
//     let mut handle = stdin.lock();

//     handle.read_to_string(&mut buffer)?;
//     Ok(())
// }