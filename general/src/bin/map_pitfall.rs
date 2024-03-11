//! Demonstrates a pitfall arising when spawning threads with the `map`` combinator. The pitfall is that `map` produces
//! an iterator, which is lazy. In order to ensure the threads are spawned correctly, use the `collect` combinator
//! after `map` to force the execution.

use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

fn main() {
    let f = |i: u64| -> JoinHandle<()> {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100 - i * 10));
            println!("{i} at {:?}", Instant::now());
        })
    };

    println!("\nmap and join, no collect");
    {
        let hs_by_map = (0..5).map(|i| f(i));
        thread::sleep(Duration::from_millis(100));
        hs_by_map.for_each(|h| h.join().unwrap());
    }

    println!("\nmap, no join, no collect");
    {
        let _hs_by_map = (0..5).map(|i| f(i));
        thread::sleep(Duration::from_millis(100));
    }

    println!("\nmap and join, with collect");
    {
        let hs_by_map: Vec<JoinHandle<()>> = (0..5).map(|i| f(i)).collect();
        thread::sleep(Duration::from_millis(100));
        hs_by_map.into_iter().for_each(|h| h.join().unwrap());
    }

    println!("\nmap, no join, with collect");
    {
        let _hs_by_map: Vec<JoinHandle<()>> = (0..5).map(|i| f(i)).collect();
        thread::sleep(Duration::from_millis(100));
    }
}
