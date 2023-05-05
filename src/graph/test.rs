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
    // Checks if plotly is in the toml file. 
    let dummy = plotly::Plot::new();

}

#[test]
pub fn check_modules() {
    let path_to_files = vec![Path::new("src/eigentrust.rs"), Path::new("src/graph/mod.rs"), Path::new("src/graph/matrix.rs"), Path::new("src/graph/file.rs")];
    for i in path_to_files {
        if !i.exists() {
            panic!("One or more of the modules are missing");
        }
    }

}

#[test]
pub fn verify_file() {
        // Given a file location read it in and return a list of edges along with their weight, and number of nodes
        let mut result: Vec<(u32, u32, i8)> = Vec::new();
        let contents =
            std::fs::read_to_string("soc-sign-bitcoinotc.csv".to_string()).expect("Should have been able to read the file"); // Reading file in as string
        for i in contents.split("\n") {
            if i != "" { // Empty line is an exception to four column rule
                if i.split(",").collect::<Vec<&str>>().len() != 4 {
                    panic!("Row(s) in soc-sign-bitcoinotc.csv do not have exactly four columns");
                }
            }
        }
}

#[test]
pub fn verify_trust() {
    let n = 4;
    let test_edges = vec![(1, 2, -10_i8), (2, 1, 10_i8), (3, 1, 10_i8), (1, 4, 10_i8), (4, 1, 10_i8)];
    let obj = super::AdjacencyMatrix::create(test_edges, 0_i8, n as u32);
    let result = super::super::eigentrust::all_walks(1000, 10, &obj);
    for i in result {
        if i != 1_u32 && i != 4_u32 {
            println!("{}", i);
            panic!("One or more functions do not work!");
        }
    }
}