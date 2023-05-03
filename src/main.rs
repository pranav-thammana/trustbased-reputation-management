use std::fs;
pub mod graph;
pub mod eigentrust;
use std::time::Duration;
use std::thread::available_parallelism;
use std::rc::Rc;
use std::{thread, mem};
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    println!("Hello World!");
    let test = graph::read_file("soc-sign-bitcoinotc.csv");
    println!("File read successfully!");
    let test1 = graph::AdjacencyMatrix::create(test.0, 0, test.1);
    println!("Adjacency Matrix created successfully!");
    // println!("{:?}", eigentrust::all_walks(1000_usize, 1000_usize, &test1));

    let thread_count: usize = available_parallelism().unwrap().get(); // Change to desired thread count
    let total_iterations: usize = 100000_usize;
    let individual_thread = total_iterations / thread_count;
    let last_thread = total_iterations as usize - individual_thread as usize*thread_count as usize;
    let cycles = 1000_usize;
    let mut thread_handles = Vec::new();
    if thread_count == 1 {
        // Insert function to analyze all_walks result and all_walks function itself to generate result
    }
    else if thread_count >= 1 {
        let obj = Arc::new(test1);
        for dummy in 1..thread_count {
            let iter_ = cycles.clone(); // Cloning numbers for ease and because it is computationally inexpensive
            let walks_ = individual_thread.clone();
            let obj_ = obj.clone();
            let thread_num = Arc::new(dummy);
            thread_handles.push(
                thread::spawn(move || {
                    println!("Thread {} began", thread_num);
                    let temp =  eigentrust::all_walks(walks_, iter_, &*obj_);
                    println!("Thread {} ended", thread_num);
                    return temp
                })
            );
        }
        // Covering code for last thread
        let iter_ = cycles.clone();
        let obj_ = obj.clone();
        let thread_count_ = Arc::new(thread_count);
        thread_handles.push(
            thread::spawn(move || {
                println!("Thread {} began", thread_count_);
                let temp = eigentrust::all_walks(last_thread, iter_, &*obj);
                println!("Thread {} ended", thread_count_);
                return temp
            })
        )
    }
    else {
        panic!("Thread_count variable must be greater than or equal to 1")
    }
    let mut result = Vec::new();
    for thread in thread_handles {
        for answer in thread.join().unwrap() {
            result.push(answer);
        }
    }
    

}

