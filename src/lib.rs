use std::fs;
use std::path::Path;

pub fn print(dir: &Path , level: usize) -> std::io::Result<()> {
    let padding = " ".repeat((level - 1) * 2);

    if let Some(dir_string) = dir.file_stem() {
        if let Some(dir_string) = dir_string.to_str() {
            println!("{}{}", padding, dir_string);
        }
    }

    let padding = " ".repeat(level * 2);

    let files = fs::read_dir(dir)?;
    for file in files {
        let file = file?;
        let file_name = file.file_name();

        if let Ok(file_name) = file_name.into_string() {
            if file_name.starts_with(".") {
                continue;
            }

            if fs::metadata(file.path())?.file_type().is_dir() {
                print(&file.path(), level + 1);
            } else {
                println!("{}{}", padding, file_name);
            }
        }
    }

    Ok(())
}
