use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

fn xdg_path_manager() -> Option<PathBuf> {
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
    return xdg_path;
}

fn homelocalshare_path_manager() -> Option<PathBuf> {
    let key: &str = "HOME";

    let value: Option<OsString> = env::var_os(key);

    let home_path: Option<PathBuf> = match value {
        Some(v) => {
            let pathnew: PathBuf = PathBuf::from(v);

            if pathnew.is_absolute() {
                Some(pathnew.join(".local/share"))
            } else {
                None
            }
        }
        None => None,
    };
    return home_path;
}

pub fn top_manager() -> Option<PathBuf> {
    let v1 = xdg_path_manager();
    let v2 = homelocalshare_path_manager();

    if v1 == None { v2 } else { v1 }
}

pub fn prefix_manager() {}
