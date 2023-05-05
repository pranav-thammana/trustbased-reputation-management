use std::collections::HashMap;
use crate::graph::AdjacencyMatrix;
use rand::distributions::{Distribution, Uniform};
use std::{thread};


fn random_walk(iters: usize, obj: &AdjacencyMatrix) -> u32 {
    /*
    Given a number of iterations and the adjacency matrix, do a random walk
    starting at a random node with iters steps
    */

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..obj.norm_trust_vec.len() as usize); // Initialize die to get initial position
    let mut pos = die.sample(&mut rng); // initial position
    for i in 0..iters {
        if obj.norm_trust_vec[pos] as usize == 0_usize { // If the node has zero ratings for others case
            pos = die.sample(&mut rng); // Jump to a random other node
        }
        else {
            /*
            Generates a random number between 0 and the positive sum of all ratings given by the node
            and uses that information to find a random node to jump to weighted by their rating
            */

            let die2 = Uniform::from(1..obj.norm_trust_vec[pos] as usize + 1_usize); // initialize die
            let roll2 = die2.sample(&mut rng);
            let mut rolling_counter = 0_usize;
            for i in 0..*&obj.obj[pos].len() {
                if &obj.obj[pos][i] >= &0_i8 {
                    rolling_counter += *&obj.obj[pos][i] as usize;
                }
                if rolling_counter >= roll2 {
                    pos = i;
                    break
                }
            }
        }
    }
    pos as u32 + 1_u32 
    /*
    Increases the position by 1 because the actual node name in the file is 1 + the index of the adjacency matrix
    ex. node #1 is index 0
    */
}

pub fn all_walks(walks: usize, _iters: usize, obj: &AdjacencyMatrix) -> Vec<u32> {
    // Creates a vector of a bunch of random walks
    let mut result: Vec<u32> = vec![];
    for _i in 0..walks {
        result.push(random_walk(_iters, obj))
    }
    return result
}

pub fn flatten(thread_handles: Vec<thread::JoinHandle<Vec<u32>>>, mut result: Vec<u32>) -> Vec<u32> {
    for thread in thread_handles {
        for answer in thread.join().unwrap() {
            result.push(answer);
        }
    }
    result
}

pub fn analysis(results: &Vec<u32>) -> Vec<(u32, u32)> {
    // Sorts results by trust value and returns five must trusted nodes and least trusted nodes
    let mut temp = HashMap::new();
    for j in results {
        let hash_entry = temp.entry(*j).or_insert(0_u32);
        *hash_entry += 1_u32;
    }
    let mut final_result: Vec<(u32, u32)> = temp.iter().map(|(x, y)| (*x, *y)).collect();
    final_result.sort_by(|a, b| b.1.cmp(&a.1)); // Requires 2nd element to be score
    println!("The five most trusted nodes are {:?}", &final_result[0..5].iter().map(|x| x.0).collect::<Vec<u32>>());
    final_result.sort_by(|a, b| a.1.cmp(&b.1)); // Requires 2nd element to be score
    println!("The five least trusted nodes with at least one positive connection are {:?}", &final_result[0..5].iter().map(|x| x.0).collect::<Vec<u32>>());
    return final_result
}