pub mod graph;
pub mod eigentrust;
use std::thread::available_parallelism;
use std::{thread};
use std::sync::{Arc};
use plotly::{Plot, Scatter};
use plotly::common::Mode;



fn main() {
    // Reads in file, creates, adjacency matrix, prepares for running of algorithm 

    let test = graph::read_file("soc-sign-bitcoinotc.csv");
    println!("File read successfully!");
    let test1 = graph::AdjacencyMatrix::create(test.0, 0, test.1); // Change offset value here if you want to
    println!("Adjacency Matrix created successfully!");
    // println!("{:?}", eigentrust::all_walks(1000_usize, 1000_usize, &test1));


    // Multithreading
    let thread_count: usize = available_parallelism().unwrap().get(); // Change to desired thread count
    let total_iterations: usize = 100000_usize; // Change to desired iteration count (total amount of random walks)
    let individual_thread = total_iterations / thread_count; // Iterations for each thread but last one
    let last_thread = total_iterations as usize - individual_thread as usize*thread_count as usize; // iterations for last thread (remainder)
    let cycles = 1000_usize; // Change to desired cycle count (amount of 'steps' in the random walk)
    let mut thread_handles = Vec::new();
    let obj = Arc::new(test1);

    for dummy in 1..thread_count {
        let iter_ = cycles.clone(); // Cloning numbers for ease and because it is computationally inexpensive
        let walks_ = individual_thread.clone();
        let obj_ = obj.clone();
        let thread_num = Arc::new(dummy);
        thread_handles.push(
            thread::spawn(move || {
                println!("Thread {} began", thread_num);
                let temp =  eigentrust::all_walks(walks_, iter_, &*obj_); // Vector with nodes at the end of each random walk
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
            let temp = eigentrust::all_walks(last_thread, iter_, &*obj_); // Dereferences arc pointer and passes in regular reference
            println!("Thread {} ended", thread_count_);
            return temp
        })
    );
    let result = Vec::new();
    let vis_data_unprepared = eigentrust::analysis(&eigentrust::flatten(thread_handles, result));
    
    // Visualizations 

    let mut data_prepared = Vec::new();
    for i in &vis_data_unprepared {
        data_prepared.push(i.1)
    }
    data_prepared.sort();
    
    // let trace = Scatter::new((1_u32..data_prepared.len() as u32 + 1_u32).collect::<Vec<u32>>(), data_prepared);
    let trace = Scatter::new((1_u32..data_prepared.len() as u32 +1_u32).collect::<Vec<u32>>(), data_prepared).mode(Mode::Markers);
    let mut plot = Plot::new(); 
    plot.add_trace(trace);
    plot.write_html("out.html"); // Creates file for graph

    


}



