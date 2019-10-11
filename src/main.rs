pub mod lib;
use lib::print;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut dir = Path::new(".");

    if let Some(_dir) = args.get(1) {
        dir = Path::new(_dir);
    }

    print(&dir, 1);
}
