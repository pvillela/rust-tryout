use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    thread::scope,
    time::{Duration, Instant},
};

/// Supports the thread-safe initialization of variables (including `static`s) once and only once.
pub struct AtomicInit<T> {
    initialized: AtomicBool,
    value: UnsafeCell<Option<T>>, // guaranteed not to be None when exists is true
}

impl<T> AtomicInit<T> {
    /// Returns Option containing value if it is visible, None otherwise.
    pub fn try_get(&self) -> Option<&T> {
        if self.initialized.load(Ordering::Acquire) {
            // Safety: value is guaranteed to be stable as initialization was controlled by atomic to
            // have happened before once and only once.
            unsafe { (&*self.value.get()).as_ref() }
        } else {
            None
        }
    }

    /// Waits in a spin-lock to get value until it is visible or timeout expires.
    pub fn get_with_timeout(&self, timeout: Duration) -> Option<&T> {
        let current = Instant::now();
        while !self.initialized.load(Ordering::Acquire) {
            std::hint::spin_loop();
            if current.elapsed().gt(&timeout) {
                return None;
            }
        }
        // Safety: value is guaranteed to be stable as initialization was controlled by atomic to
        // have happened before once and only once.
        unsafe { (&*self.value.get()).as_ref() }
    }

    /// Gets the value if initialized, panics otherwise.
    pub fn get(&self) -> &T {
        // Fast path.
        if let Some(r) = self.try_get() {
            return r;
        }

        // Slower path.

        let swap =
            self.initialized
                .compare_exchange(true, true, Ordering::Acquire, Ordering::Relaxed);
        if swap.is_err() {
            panic!("Access to uninitialized value");
        }
        // Safety: value is guaranteed to be stable as initialization was controlled by atomic to
        // have happened before once and only once.
        unsafe { (&*self.value.get()).as_ref().unwrap() }
    }

    /// Creates an empty instance of Self.
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(None),
        }
    }

    /// Initializes self with with a given value.
    pub fn init(&self, value: T) -> Result<(), ()> {
        let swap =
            self.initialized
                .compare_exchange(false, true, Ordering::Release, Ordering::Relaxed);
        if swap.is_err() {
            return Err(());
        }
        // Safety: value initialization is controlled by atomic to happen once and only once.
        unsafe {
            let p = self.value.get();
            *p = Some(value);
        }
        Ok(())
    }
}

// Safety: value is guaranteed to be stable as initialization is controlled by atomic to
// have happened once and only once before any access to the UnsafeCell.
unsafe impl<T: Sync> Sync for AtomicInit<T> {}

static VALUE: AtomicInit<String> = AtomicInit::new();

fn main() {
    println!("{:?}", VALUE.try_get());
    println!("{:?}", VALUE.get_with_timeout(Duration::from_millis(10)));
    assert!(VALUE.init("foo".to_owned()).is_ok());
    // assert!(VALUE.init("bar".to_owned()).is_ok()); // duplicate initialization attempt fails

    scope(|s| {
        s.spawn(|| {
            println!("{:?}", VALUE.try_get());
            println!("{:?}", VALUE.get_with_timeout(Duration::from_millis(10)));
            println!("{:?}", VALUE.get());
        });
    });
}
