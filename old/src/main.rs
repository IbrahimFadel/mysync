extern crate mysync;

use mysync::prints::welcome::welcome;
use mysync::inputs::input::input;

fn main() {    
    welcome();

    let mut platform = String::new();
    input("What platform are you on? (MAC/PC) | (1/2)", &mut platform);
    print!("{}", platform);

    let mut school: bool = true;

    let num1 = String::from("1");
    if platform == num1 {
        println!("Hi!");
    }
    // if let "1" = &*platform {
        // println!("Hi");
    // }
    // if platform == "1" {
    //     school = true;
    // } else if platform == "2" {
    //     school = false;
    // } else {
    //     println!("Error, invalid input");
    //     return;
    // }
}
