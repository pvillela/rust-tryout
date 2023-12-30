//! This trait extends [`DerefMut`] with an [`Option`] target, adding `unwrap` and `unwrap_mut` methods.

use std::{cell::RefCell, fmt::Debug, ops::DerefMut, sync::Mutex};

pub trait DerefMutOption<T>: DerefMut<Target = Option<T>> {
    fn unwrap(&self) -> &T;

    fn unwrap_mut(&mut self) -> &mut T;
}

impl<T, X> DerefMutOption<T> for X
where
    X: DerefMut<Target = Option<T>>,
{
    fn unwrap(&self) -> &T {
        self.as_ref().unwrap()
    }

    fn unwrap_mut(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }
}

fn foo<T: Debug>(x: &mut impl DerefMutOption<T>) {
    println!("{:?}", x.unwrap());
    println!("{:?}", x.unwrap_mut());
}

fn main() {
    let x = Mutex::new(Some(1));
    let mut guard = x.lock().unwrap();
    foo(&mut guard);
    let uguard = guard.unwrap_mut();
    *uguard += 10;
    foo(&mut guard);

    let x = RefCell::new(Some(2));
    let mut guard = x.borrow_mut();
    foo(&mut guard);
    let uguard = guard.unwrap_mut();
    *uguard += 10;
    foo(&mut guard);
}
