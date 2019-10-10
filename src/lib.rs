use std::fs;
use std::path::Path;

pub fn print(args: Vec<String>) -> std::io::Result<()> {
    let mut dir = Path::new(".");

    if let Some(_dir) = args.get(1) {
        dir = Path::new(_dir);
    }

    if let Some(dir_string) = dir.to_str() {
        println!("{}", dir_string);
    }

    let files = fs::read_dir(dir)?;
    for file in files {
        let file = file?.file_name();
        if let Ok(file_name) = file.into_string() {
            println!("{}", file_name);
        }
    }

    Ok(())
}
