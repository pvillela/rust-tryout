#![allow(unused)]

use core::time::Duration;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
struct DataStructure {
    pub x: String,
    pub y: i32,
}

// Create an atomic reference counted pointer to an atomic pointer that holds the data structure
static DATA_STRUCTURE: Lazy<Mutex<DataStructure>> = Lazy::new(|| {
    Mutex::new(DataStructure {
        x: "abc".to_owned(),
        y: 100,
    })
});

fn main() {
    // Create a new thread that reads from the data structure
    let handle = thread::spawn(move || {
        // Use the data structure here
        let cell_guard = DATA_STRUCTURE.lock().unwrap();
        println!("From new thread: {:?}", cell_guard);
    });

    thread::sleep(Duration::from_millis(500));

    // Use the data structure in the main thread, before update
    let cell_guard = DATA_STRUCTURE.lock().unwrap();
    println!("Use from main thread before update: {:?}", cell_guard);
    drop(cell_guard);

    // Update the data structure
    let mut cell_guard = DATA_STRUCTURE.lock().unwrap();
    *cell_guard = DataStructure {
        x: "def".to_owned(),
        y: 42,
    };
    drop(cell_guard);

    // Use the data structure in the main thread, after update
    let cell_guard = DATA_STRUCTURE.lock().unwrap();
    println!("Use from main thread after update: {:?},", cell_guard);

    handle.join().unwrap();
}
