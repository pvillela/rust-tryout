use std::{borrow::Borrow, fmt::Debug, ops::Deref, rc::Rc, sync::Arc};

/// Generic wrapper to enable the addition of new methods to the wrapped type.
#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
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
