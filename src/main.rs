fn main() {
    use std::env;
    use std::ffi::OsString;
    use std::path::PathBuf;
    let key: &str = "XDG_DATA_HOME";

    let value: Option<OsString> = env::var_os(key);

    let xdg_path: Option<PathBuf> = match value {
        Some(v) => {
            let pathnew: PathBuf = PathBuf::from(v);

            if pathnew.is_absolute() {
                Some(pathnew)
            } else {
                None
            }
        }
        None => None,
    };
    println!("{:?}", xdg_path);
}
