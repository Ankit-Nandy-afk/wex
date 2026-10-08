use std::fs;
use std::io::ErrorKind;
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
                    Err(reason) => {
                        match reason.kind() {
                            ErrorKind::NotFound => println!(
                                "Wine is not installed. Download it from https://www.winehq.org/download or install it with your package manager (apt, dnf, pacman)."
                            ),
                            ErrorKind::PermissionDenied => println!(
                                "Wine was found, but your system won't let wex run it. Check that the wine file is executable and that you are allowed to use it."
                            ), //everything else here _ i can forget the meaninng
                            _ => println!("Could not run wine: {}", reason),
                        }
                    }
                }
            }
            Err(reason) => println!("Could not create folder: {}", reason),
        },
        None => println!("No prefix path was produced"),
    }
}
