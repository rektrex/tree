pub mod lib;
use lib::print;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    print(args, 1);
}
