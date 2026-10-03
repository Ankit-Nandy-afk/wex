use std::path::Path;
use std::path::PathBuf;

fn main() {
    let path_assigned: &str = "Aything.exe";
    let path_converted: &Path = Path::new(path_assigned);

    let store_prefix: Option<PathBuf> = wex::prefix_manager(path_converted);
    println!("{:?}", store_prefix);
}
