use std::fs;
use std::path::Path;

pub fn print(dir: &Path , level: usize, mut dir_count: usize, mut file_count: usize) -> std::io::Result<(usize, usize)> {
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
                dir_count += 1;
                match print(&file.path(), level + 1, dir_count, file_count) {
                    Ok((dc, fc)) => {
                        dir_count = dc;
                        file_count = fc;
                    },
                    Err(_e) => {}
                }
            } else {
                file_count += 1;
                println!("{}{}", padding, file_name);
            }
        }
    }

    Ok((dir_count, file_count))
}
