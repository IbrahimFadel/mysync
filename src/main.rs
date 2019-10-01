extern crate mysync;

use mysync::prints::welcome::welcome;
use mysync::prints::options::options;

use mysync::inputs::input::input;

use mysync::ec2::tree_instance::tree_instance;
use mysync::ec2::upload_file_or_directory::upload_file_or_directory;

// use mysync::constants::constants::setPlatform;

fn main() {
    welcome();
    let mut platform = String::new();

    input("What platform are you on? (MAC/PC)", &mut platform);

    if platform.trim() != "mac" && platform.trim() != "pc" {
        println!("Error, Invalid platform");
        return;
    }

    options();
    
    let mut action = String::new();
    input("", &mut action);

    if action.trim() == "1" {
        tree_instance();
    } else if action.trim() == "2" {
        upload_file_or_directory(platform);
    }
}