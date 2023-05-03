use std::collections::HashMap;
use crate::graph::AdjacencyMatrix;
use rand::distributions::{Distribution, Uniform};
use std::{thread, mem};


fn random_walk(iters: usize, obj: &AdjacencyMatrix) -> u32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..obj.norm_trust_vec.len() as usize); // Initialize die to get initial position
    let mut pos = die.sample(&mut rng); // initial position
    for i in 0..iters {
        if obj.norm_trust_vec[i] as usize == 0_usize {
            pos = die.sample(&mut rng);
        }
        else {
            let die2 = Uniform::from(1..obj.norm_trust_vec[i] as usize + 1_usize); // initialize die
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
}

pub fn all_walks(walks: usize, iters: usize, obj: &AdjacencyMatrix) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    for i in 0..walks {
        result.push(random_walk(1000_usize, obj))
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

pub fn analysis(results: &Vec<u32>, obj: &AdjacencyMatrix) {
    let mut temp = HashMap::new();
    for j in results {
        let HashEntry = temp.entry(*j).or_insert(0_u32);
        *HashEntry += 1_u32;
    }
    let mut final_result: Vec<(&u32, &u32)> = temp.iter().collect();
    final_result.sort_by(|a, b| b.1.cmp(a.1)); // Requires 2nd element to be score
    println!("The five most trusted nodes are {:?}", &final_result[0..5].iter().map(|x| *x.0).collect::<Vec<u32>>());
    final_result.sort_by(|a, b| a.1.cmp(b.1)); // Requires 2nd element to be score
    println!("The five least trusted nodes are {:?}", &final_result[0..5].iter().map(|x| *x.0).collect::<Vec<u32>>());

    

}