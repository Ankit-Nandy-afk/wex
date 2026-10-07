use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let path_assigned: &str = "Aything.exe";
    let path_converted: &Path = Path::new(path_assigned);

    let store_prefix: Option<PathBuf> = wex::prefix_manager(path_converted);

    match store_prefix {
        Some(path) => match fs::create_dir_all(&path) {
            Ok(()) => {
                println!("Folder ready: {}", path.display());

                match wex::init_prefix(&path) {
                    Ok(status) => println!("wineboot finished, success: {}", status.success()),
                    Err(reason) => println!("Could not run wine: {}", reason),
                }
            }
            Err(reason) => println!("Could not create folder: {}", reason),
        },
        None => println!("No prefix path was produced"),
    }
}
