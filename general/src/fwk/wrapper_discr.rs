//! Explorations on a more general version of [`super::wrapper`] that has an additional
//! discriminant type parameter.

use std::{
    borrow::Borrow,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

//=================
// Wrapper

/// Generic wrapper to enable the addition of new methods to the wrapped type,
/// with a discriminant type parameter `P` to enable different wrappings of the same target.
#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Wrapper<T, P = ()>(pub T, PhantomData<P>);

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Wrapper<T, ()> {
        Self::constr(value)
    }
}

impl<T, P> Wrapper<T, P> {
    pub fn constr(value: T) -> Wrapper<T, P> {
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

impl<T, P> DerefMut for Wrapper<T, P> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
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

impl<T, P> IntoIterator for Wrapper<T, P>
where
    T: IntoIterator,
{
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, P> IntoIterator for &'a Wrapper<T, P>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, P> IntoIterator for &'a mut Wrapper<T, P>
where
    &'a mut T: IntoIterator,
{
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

//=================
// Mappable

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MappableDiscr<P>(P);

/// Specializetion of [Wrapper] that adds a [`map`](Self::map) method.
pub type Mappable<T, P = ()> = Wrapper<T, MappableDiscr<P>>;

impl<T> Mappable<T> {
    pub fn new(value: T) -> Mappable<T, ()> {
        Self::constr(value)
    }
}

impl<T, P> Mappable<T, P> {
    pub fn wrap(value: T) -> Mappable<T, P> {
        Self::constr(value)
    }

    /// Transforms `self` into a target [`Mappable<U, P>`] whose wrapped value is the result of applying `f` to
    /// `self`'s wrapped value.
    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> Mappable<U, P> {
        Mappable::wrap(f(&self.0))
    }
}

pub type Mappable1<T> = Mappable<T, String>;

impl<T> Mappable1<T> {
    pub fn new(value: T) -> Mappable1<T> {
        Self::constr(value)
    }

    pub fn map_str(&self, f: impl FnMut(&T) -> String) -> Mappable1<String> {
        self.map(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XMappableDiscr<P>(P);

/// Specializetion of [Wrapper] that adds a [`map`](Self::map) method.
pub type XMappable<T, P = ()> = Wrapper<T, XMappableDiscr<P>>;

impl<T> XMappable<T> {
    pub fn new(value: T) -> XMappable<T, ()> {
        Self::constr(value)
    }
}

impl<T, P> XMappable<T, P> {
    pub fn wrap(value: T) -> XMappable<T, P> {
        Self::constr(value)
    }

    /// Transforms `self` into a target [`XMappable<U, P>`] whose wrapped value is the result of applying `f` to
    /// `self`'s wrapped value.
    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> XMappable<U, P> {
        XMappable::wrap(f(&self.0))
    }
}
