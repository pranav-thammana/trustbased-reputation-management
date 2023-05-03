use std::path::Path;

#[cfg(test)]

#[test]
pub fn check_file() {
    let path_to_file = Path::new("soc-sign-bitcoinotc.csv");
    if path_to_file.exists() == false {
        panic!("File soc-sign-bitcoinotc.csv is not in directory or cannot be found. Ensure file is in the cargo project, but outside of source");
    }
}

#[test]
pub fn check_toml() {
    // Checks if rand is called in the toml file. If it is the code will execute successfully, else it will panic
    let mut rng = rand::thread_rng();
}