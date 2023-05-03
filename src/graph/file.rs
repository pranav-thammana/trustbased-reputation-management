pub fn read_file(dir: &str) -> (Vec<(u32, u32, i8)>, u32) {
    // Given a file location read it in and return a list of edges along with their weight, and number of nodes
    let mut result: Vec<(u32, u32, i8)> = Vec::new(); // List of edges
    let contents =
        std::fs::read_to_string(dir.to_string()).expect("Should have been able to read the file"); // Reading file in as string
    let mut node_count: u32 = 0;
    for line in contents.split("\n") { // Going through each line
        match line {
            "" => { // If it's an empty line we ignore
                continue;
            }
            _ => { // Otherwise we parse
                let line_split: Vec<&str> = line.split(",").collect::<Vec<&str>>(); // collect returns a Result
                if line_split.len() != 4 { // Check if file has a line with anything other than 4 columns
                    panic!("The file doesn't have four columns")
                }
                result.push((
                    line_split[0].parse().expect("Failed to parse"), // Rater
                    line_split[1].parse().expect("Failed to parse"), // Ratee
                    line_split[2].parse().expect("Failed to parse"), // Score
                ));
                if line_split[1].parse::<u32>().unwrap() > node_count {
                    node_count = line_split[1].parse::<u32>().unwrap()
                }
                if line_split[0].parse::<u32>().unwrap() > node_count {
                    node_count = line_split[1].parse::<u32>().unwrap()
                }
            }
        }
    }
    (result, node_count)
}
