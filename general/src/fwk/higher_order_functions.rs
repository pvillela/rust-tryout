use std::{borrow::Borrow, ops::Deref, sync::Arc};

/// Function that can be used as a placeholder for a configuration source during development.
/// Supports any configuration info type and panics if called.
pub fn nil_app_cfg<T>() -> Arc<T> {
    todo!("Configuration source not provided.")
}

/// Transforms a value into a nullary closure that returns the value.
pub fn const_closure<T: Clone>(x: T) -> impl Fn() -> T {
    move || x.clone()
}

/// Composes a nullary closure with another closure.
pub fn compose_nullary<S, T, F, G>(f: F, g: G) -> impl Fn() -> T
where
    F: Fn() -> S,
    G: Fn(S) -> T,
{
    move || g(f())
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns the composition of the second and third arguments.
pub fn const_or_compose<S: 'static, T: 'static + Clone, F, G>(
    k: Option<T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> T>
where
    F: 'static + Fn() -> S,
    G: 'static + Fn(S) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(k)),
        None => Box::new(compose_nullary(f, g)),
    }
}

/// Composes a nullary closure with another closure.
pub fn compose_nullary_by_ref<S, T, F, G>(f: F, g: G) -> impl Fn() -> T
where
    F: Fn() -> S,
    G: Fn(&S) -> T,
{
    move || g(&f())
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns the composition of the second and third arguments.
pub fn const_or_compose_by_ref<S: 'static, T: 'static + Clone, F, G>(
    k: Option<T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> T>
where
    F: 'static + Fn() -> S,
    G: 'static + Fn(&S) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(k)),
        None => Box::new(compose_nullary_by_ref(f, g)),
    }
}

pub fn adapt_by_ref<S, T: Clone, F, G>(f: F, g: G) -> Box<dyn Fn() -> Arc<T>>
where
    F: 'static + Fn() -> Arc<S>,
    G: 'static + Fn(&S) -> T,
{
    let h = move || Arc::new(g(f().deref()));
    Box::new(h)
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns [adapt_by_ref] of the second and third arguments.
pub fn const_or_adapt_by_ref<S, T: 'static + Clone, F, G>(
    k: Option<&T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> Arc<T>>
where
    F: 'static + Fn() -> Arc<S>,
    G: 'static + Fn(&S) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(Arc::new((*k).clone()))),
        None => Box::new(adapt_by_ref(f, g)),
    }
}

/// Composes a nullary closure with another closure.
fn compose_nullary_by_borrow<S, T, F, G>(f: F, g: G) -> impl Fn() -> T
where
    F: Fn() -> S,
    G: Fn(&dyn Borrow<S>) -> T,
{
    move || g(f().borrow())
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns the composition of the second and third arguments.
pub fn const_or_compose_by_borrow<S: 'static, T: 'static + Clone, F, G>(
    k: Option<T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> T>
where
    F: 'static + Fn() -> S,
    G: 'static + Fn(&dyn Borrow<S>) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(k)),
        None => Box::new(compose_nullary_by_borrow(f, g)),
    }
}
