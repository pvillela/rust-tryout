#![allow(unused)]

use arc_swap::ArcSwap;
use core::time::Duration;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct DataStructure {
    pub x: String,
    pub y: i32,
}

// Create an atomic reference counted pointer to an atomic pointer that holds the data structure
static DATA_STRUCTURE: Lazy<ArcSwap<DataStructure>> = Lazy::new(|| {
    ArcSwap::from_pointee(DataStructure {
        x: "abc".to_owned(),
        y: 100,
    })
});

fn main() {
    // Create a new thread that reads from the data structure
    let handle = thread::spawn(move || {
        // Use the data structure here
        let data_structure = DATA_STRUCTURE.load();
        println!("From new thread: {:?}", data_structure);
    });

    thread::sleep(Duration::from_millis(500));

    // Use the data structure in the main thread, before update
    let data_structure = DATA_STRUCTURE.load();
    println!("Use from main thread before update: {:?}", data_structure);

    // Update the data structure
    let new_data_structure = Arc::new(DataStructure {
        x: "def".to_owned(),
        y: 42,
    });
    DATA_STRUCTURE.store(new_data_structure);

    // Use the data structure in the main thread, after update
    let data_structure = DATA_STRUCTURE.load();
    println!("Use from main thread after update: {:?}", data_structure);

    handle.join().unwrap();
}
