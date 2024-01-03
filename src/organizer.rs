use std::fs;
use std::path::Path;

pub fn organize_files(directory: &str) {
    let base_path = Path::new(directory);
    let paths = fs::read_dir(base_path).expect("Unable to read directory");

    for path in paths {
        match path {
            Ok(entry) => {
                let path = entry.path();
                let file_type = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                match file_type {
                    "jpg" | "png" => move_file(base_path, &path, "Images"),
                    "mp3" => move_file(base_path, &path, "Music"),
                    "mov" => move_file(base_path, &path, "Videos"),
                    "dmg" => move_file(base_path, &path, "Installed Packages"),
                    "pdf" => move_file(base_path, &path, "Docs"),
                    _ => (),
                }
            }
            Err(e) => println!("Error reading path: {}", e),
        }
    }
}

fn move_file(base_dir: &Path, path: &Path, category: &str) {
    // Construct the target directory path within the base directory
    let target_dir = base_dir.join(category);
    if !target_dir.exists() {
        fs::create_dir(&target_dir).expect("Unable to create directory");
    }

    if let Some(filename) = path.file_name() {
        let target_path = target_dir.join(filename);
        fs::rename(path, &target_path).expect("Unable to move file");
        println!("Moved {:?} to {:?}", path, target_path);
    } else {
        println!("Failed to get the file name for {:?}", path);
    }
}