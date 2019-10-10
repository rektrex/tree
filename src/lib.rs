use std::fs;
use std::path::Path;

pub fn print(args: Vec<String>) -> std::io::Result<()> {
    let mut dir_count = 0;
    let mut file_count = 0;

    let mut dir = Path::new(".");

    if let Some(_dir) = args.get(1) {
        dir = Path::new(_dir);
    }

    if let Some(dir_string) = dir.to_str() {
        println!("{}", dir_string);
    }

    let files = fs::read_dir(dir)?;
    for file in files {
        let file = file?;
        let file_name = file.file_name();
        if let Ok(file_name) = file_name.into_string() {
            if fs::metadata(file.path())?.file_type().is_dir() {
                dir_count += 1;
            } else {
                file_count += 1;
            }
            println!("{}", file_name);
        }
    }

    println!("\n{} directories, {} files", dir_count, file_count);

    Ok(())
}
