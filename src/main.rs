fn main() {
    use std::env;
    use std::ffi::OsString;
    let key: &str = "HOME";
    let ans: Option<OsString> = env::var_os(key);
    println!("Found it {:?}", ans);
}
