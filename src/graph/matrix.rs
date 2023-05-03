pub struct AdjacencyMatrix {
    // Adjacency matrix struct is designed such that indexing by row is quickest
    pub obj: Vec<Vec<i8>>, // Actual matrix. A[i][j] is the ith raters rating for j user
    pub norm_trust_vec: Vec<u64>, // Short for normalized trust score denominator. The ith value is the sum of all elements greater than zero of the ith row of the matrix
}

impl AdjacencyMatrix {
    pub fn create(list: Vec<(u32, u32, i8)>, default: i8, n: u32) -> AdjacencyMatrix {
        let mut result: Vec<Vec<i8>> = vec![vec![default; n as usize]; n as usize];
        for i in list {
            result[i.0 as usize - 1][i.1 as usize - 1] = i.2 // nth node is the n-1 index
        }
        let mut ntd: Vec<u64> = Vec::new();
        for i in 0..*&result.len() {
            let mut sum: u64 = 0_u64;
            for j in 0..*&result.len() {
                if &result[i][j] > &0_i8 {
                    sum += *&result[i][j] as u64;
                }
            }
            ntd.push(sum)
        }
        return AdjacencyMatrix {
            obj: result,
            norm_trust_vec: ntd
        };
    }
}