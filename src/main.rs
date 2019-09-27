extern crate mysync;

use mysync::prints::welcome::welcome;
use mysync::prints::options::options;

use mysync::inputs::input::input;

use mysync::ec2::tree_instance::tree_instance;

fn main() {
    welcome();

    let mut platform = String::new();

    input("What platform are you on? (MAC/PC) | (1/2)", &mut platform);

    if platform.trim() == "1" {
        println!("MAC");
    } else if platform.trim() == "2" {
        println!("PC");
    } else {
        println!("Error, invalid input"); 
    }

    options();
    
    let mut action = String::new();
    input("", &mut action);

    if action.trim() == "1" {
        tree_instance();
    }
}