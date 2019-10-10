use std::fs;
use std::path::Path;

pub fn print(args: Vec<String>) {
    let mut dir = Path::new(".");

    if let Some(_dir) = args.get(1) {
        dir = Path::new(_dir);
    }

    if let Some(dir_string) = dir.to_str() {
        println!("{}", dir_string);
    }

    if let Ok(files) = fs::read_dir(dir) {
        for file in files {
            if let Ok(file) = file {
                if let Ok(file_name) = file.file_name().into_string() {
                    println!("{}", file_name);
                }
            }
        }
    }
}
