use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

pub struct AtomicGuard<'a, T> {
    exists: AtomicBool,
    reference: &'a mut Option<T>, // guaranteed not to be None when exists is true
}

impl<'a, T> AtomicGuard<'a, T> {
    /// Returns Option containing value if it is visible, None otherwise.
    pub fn try_get(&'a self) -> Option<&'a T> {
        if self.exists.load(Ordering::Acquire) {
            self.reference.as_ref()
        } else {
            None
        }
    }

    /// Waits in a spin-lock to get value until it is visible or timeout expires.
    pub fn get_with_timeout(&'a self, timeout: Duration) -> Option<&'a T> {
        let current = Instant::now();
        while !self.exists.load(Ordering::Acquire) {
            std::hint::spin_loop();
            if current.elapsed().gt(&timeout) {
                return None;
            }
        }
        self.reference.as_ref()
    }

    /// Gets the value if initialized, panics otherwise.
    pub fn get(&'a self) -> &'a T {
        let _ = self
            .exists
            .compare_exchange(true, true, Ordering::Acquire, Ordering::Relaxed);
        self.reference.as_ref().unwrap()
    }

    pub fn new(reference: &'a mut Option<T>) -> Self {
        Self {
            exists: AtomicBool::new(false),
            reference,
        }
    }

    pub fn init(&mut self, value: T) -> Result<(), ()> {
        let swap = self
            .exists
            .compare_exchange(false, true, Ordering::Release, Ordering::Relaxed);
        if swap.is_err() {
            return Err(());
        }
        *self.reference = Some(value);
        Ok(())
    }
}

static mut VALUE: Option<String> = None;

fn main() {
    let mut ag = unsafe { AtomicGuard::new(&mut VALUE) };
    println!("{:?}", ag.try_get());
    println!("{:?}", ag.get_with_timeout(Duration::from_millis(10)));
    assert!(ag.init("foo".to_owned()).is_ok());
    // assert!(ag.init("bar".to_owned()).is_ok()); // duplicate initialization attempt fails
    println!("{:?}", ag.try_get());
    println!("{:?}", ag.get_with_timeout(Duration::from_millis(10)));
    println!("{:?}", ag.get());
}
