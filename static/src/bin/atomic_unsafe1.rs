#![allow(unused)]

use core::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct DataStructure {
    pub x: String,
    pub y: i32,
}

// Create an atomic reference counted pointer to the data structure
static DATA_STRUCTURE: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // Initialize the data structure
    let orig_data_structure = Arc::new(DataStructure {
        x: "abc".to_owned(),
        y: 100,
    });

    // Store the data structure in the static variable using an atomic store
    DATA_STRUCTURE.store(Arc::as_ptr(&orig_data_structure) as usize, Ordering::SeqCst);

    // Create a new thread that reads from the data structure
    let handle = thread::spawn(move || {
        let ptr = DATA_STRUCTURE.load(Ordering::SeqCst);
        let data_structure = unsafe { Arc::from_raw(ptr as *const DataStructure) };

        // Use the data structure in the new thread
        println!("From new thread: {:?}", data_structure);
    });

    thread::sleep(Duration::from_millis(500));

    // Another instance of data structure
    let new_data_structure = Arc::new(DataStructure {
        x: "def".to_owned(),
        y: 42,
    });

    // Store the data structure in the static variable using an atomic store
    DATA_STRUCTURE.store(Arc::as_ptr(&new_data_structure) as usize, Ordering::SeqCst);

    // Use the data structure in the main thread
    let ptr = DATA_STRUCTURE.load(Ordering::SeqCst);
    let data_structure = unsafe { Arc::from_raw(ptr as *const DataStructure) };
    println!("From main thread: {:?}", data_structure);

    handle.join().unwrap();
}
