use std::{borrow::Borrow, fmt::Debug, marker::PhantomData, ops::Deref, rc::Rc, sync::Arc};

//=================
// Wrapper

/// Generic wrapper to enable the addition of new methods to the wrapped type.
#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Wrapper<T, P = ()>(pub T, PhantomData<P>);

impl<T, P> Wrapper<T, P> {
    pub fn wrap(value: T) -> Wrapper<T, P> {
        Self(value, PhantomData)
    }

    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T, P> Debug for Wrapper<T, P>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.0 as &dyn Debug).fmt(f)
    }
}

impl<T, P> From<T> for Wrapper<T, P> {
    fn from(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T, P> Deref for Wrapper<T, P> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, P> AsRef<T> for Wrapper<T, P> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T, P> Borrow<T> for Wrapper<T, P> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T, P> Borrow<T> for Wrapper<Box<T>, P> {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T, P> Borrow<T> for Wrapper<Arc<T>, P> {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T, P> Borrow<T> for Wrapper<Rc<T>, P> {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

//=================
// Mappable

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MappablePhantom;

/// Specializetion of [Wrapper] that adds a [`map`](Self::map) method.
pub type Mappable<T> = Wrapper<T, MappablePhantom>;

impl<T> Mappable<T> {
    /// Transforms `self` into a target [`Mappable<U>`] whose wrapped value is the result of applying `f` to
    /// `self`'s wrapped value.
    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> Mappable<U> {
        Wrapper::wrap(f(&self.0))
    }
}
