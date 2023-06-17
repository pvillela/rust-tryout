use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

pub struct AtomicGuard<T: Clone> {
    exists: AtomicBool,
    value: Option<T>, // guaranteed not to be None when exists is true
}

impl<T: Clone> AtomicGuard<T> {
    /// Returns Option containing value if it is vilible, None otherwise.
    pub fn try_get(&self) -> Option<T> {
        if self.exists.load(Ordering::Acquire) {
            self.value.clone()
        } else {
            None
        }
    }

    /// Waits in a spin-lock to get value until it is visible or timeout expires.
    pub fn get_with_timeout(&self, timeout: Duration) -> Option<T> {
        let current = Instant::now();
        while !self.exists.load(Ordering::Acquire) {
            if current.elapsed().gt(&timeout) {
                return None;
            }
        }
        self.value.clone()
    }

    /// Waits forever a spin-lock to get value until it is visible.
    pub fn get(&self) -> T {
        self.get_with_timeout(Duration::MAX).unwrap()
    }

    pub fn new() -> AtomicGuard<T> {
        AtomicGuard {
            exists: AtomicBool::new(false),
            value: None,
        }
    }

    pub fn init(&mut self, value: T) -> Result<(), ()> {
        let swap = self
            .exists
            .compare_exchange(false, true, Ordering::Release, Ordering::Relaxed);
        if !swap.is_ok() {
            return Err(());
        }
        self.value = Some(value);
        Ok(())
    }
}

fn main() {
    let mut ag = AtomicGuard::<String>::new();
    println!("{:?}", ag.try_get());
    println!("{:?}", ag.get_with_timeout(Duration::from_millis(10)));
    assert!(ag.init("foo".to_owned()).is_ok());
    // assert!(ag.init("bar".to_owned()).is_ok()); // duplicate initialization attempt fails
    println!("{:?}", ag.try_get());
    println!("{:?}", ag.get_with_timeout(Duration::from_millis(10)));
    println!("{:?}", ag.get());
}
