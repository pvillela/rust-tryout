use core::time::Duration;
use once_cell::sync::Lazy;
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};
use std::thread;

#[derive(Debug)]
struct DataStructure {
    pub x: String,
    pub y: i32,
}

// Create an atomic reference counted pointer to an atomic pointer that holds the data structure
static DATA_STRUCTURE: Lazy<Arc<AtomicPtr<DataStructure>>> = Lazy::new(|| {
    Arc::new(AtomicPtr::new(Box::into_raw(Box::new(DataStructure {
        x: "abc".to_owned(),
        y: 100,
    }))))
});

fn main() {
    let data_structure_copy = DATA_STRUCTURE.clone();

    // Create a new thread that reads from the data structure
    let handle = thread::spawn(move || {
        let ptr = data_structure_copy.load(Ordering::Acquire);
        let data_structure = unsafe { &*ptr };

        // Use the data structure here
        println!("From new thread: {:?}", data_structure);
    });

    thread::sleep(Duration::from_millis(500));

    // Use the data structure in the main thread, before update
    let ptr = DATA_STRUCTURE.load(Ordering::Acquire);
    let data_structure = unsafe { &*ptr };
    println!("Use from main thread before update: {:?}", data_structure);

    // Update the data structure
    let new_data_structure = Box::new(DataStructure {
        x: "def".to_owned(),
        y: 42,
    });
    let new_ptr = Box::into_raw(new_data_structure);
    DATA_STRUCTURE.store(new_ptr, Ordering::Release);

    // Use the data structure in the main thread, after update
    let ptr = DATA_STRUCTURE.load(Ordering::Acquire);
    let data_structure = unsafe { &*ptr };
    println!("Use from main thread after update: {:?}", data_structure);

    handle.join().unwrap();
}
