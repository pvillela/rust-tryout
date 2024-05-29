//! Generic wrapper to facilitate the addition of new methods to the wrapped type.

use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

/// Generic wrapper to facilitate the addition of new methods to the wrapped type.
#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Wrapper<T> {
        Self(value)
    }

    pub fn wrap(value: T) -> Wrapper<T> {
        Self(value)
    }

    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T> Debug for Wrapper<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.0 as &dyn Debug).fmt(f)
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsRef<T> for Wrapper<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Borrow<T> for Wrapper<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> Borrow<T> for Wrapper<Box<T>> {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T> Borrow<T> for Wrapper<Arc<T>> {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T> Borrow<T> for Wrapper<Rc<T>> {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T> IntoIterator for Wrapper<T>
where
    T: IntoIterator,
{
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Wrapper<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Wrapper<T>
where
    &'a mut T: IntoIterator,
{
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
