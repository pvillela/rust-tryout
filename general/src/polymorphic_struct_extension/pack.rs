//! This submodule demonstrates the ***parameter packing*** pattern (see [parent module](super)).

use arc_swap::{ArcSwap, ArcSwapAny};
use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Rc,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
};

pub trait InnerMut<I> {
    fn get_inner_clone(&self) -> I;

    fn set_inner(&self, inner: I);
}

impl<T> InnerMut<Arc<T>> for ArcSwap<T> {
    fn get_inner_clone(&self) -> Arc<T> {
        self.load().clone()
    }

    fn set_inner(&self, inner: Arc<T>) {
        self.store(inner);
    }
}

impl<I: Clone> InnerMut<I> for RefCell<I> {
    fn get_inner_clone(&self) -> I {
        self.borrow().clone()
    }

    fn set_inner(&self, inner: I) {
        self.replace(inner);
    }
}

pub mod a {
    use super::*;

    pub struct Cfg<T, TX, IM>
    where
        T: 'static,
        TX: From<T> + Clone,
        IM: InnerMut<TX>,
    {
        cache: IM,
        refresh: fn() -> T,
        _tx: PhantomData<TX>,
    }

    impl<T, TX, IM> Cfg<T, TX, IM>
    where
        T: 'static,
        TX: From<T> + Clone,
        IM: InnerMut<TX>,
    {
        pub fn get(&self) -> TX {
            self.cache.get_inner_clone()
        }

        pub fn refresh(&self) {
            self.cache.set_inner((self.refresh)().into())
        }
    }

    pub type CfgRefCellRc<T> = Cfg<T, Rc<T>, RefCell<Rc<T>>>;

    impl<T> CfgRefCellRc<T> {
        pub fn new(refresh: fn() -> T) -> Self {
            let cfg = Self {
                cache: RefCell::new(Rc::new(refresh())),
                refresh,
                _tx: PhantomData,
            };
            cfg
        }
    }

    pub type CfgArcSwapArc<T> = Cfg<T, Arc<T>, ArcSwap<T>>;

    impl<T> CfgArcSwapArc<T> {
        pub fn new(refresh: fn() -> T) -> Self {
            let cfg = Self {
                cache: ArcSwapAny::new(Arc::new(refresh())),
                refresh,
                _tx: PhantomData,
            };
            cfg
        }
    }
}

pub mod b {
    use super::*;

    pub trait Param {
        type T: 'static;
        type TX: From<Self::T> + Clone;
        type IM: InnerMut<Self::TX>;
    }

    pub struct Cfg<P: Param> {
        cache: P::IM,
        refresh: fn() -> P::T,
    }

    impl<P: Param> Cfg<P> {
        pub fn get(&self) -> P::TX {
            self.cache.get_inner_clone()
        }

        pub fn refresh(&self) {
            self.cache.set_inner((self.refresh)().into())
        }
    }

    pub struct RefCellRcP<T: 'static>(PhantomData<T>);

    impl<T> Param for RefCellRcP<T> {
        type T = T;
        type TX = Rc<T>;
        type IM = RefCell<Rc<T>>;
    }

    pub struct ArcSwapArcP<T: 'static>(PhantomData<T>);

    impl<T> Param for ArcSwapArcP<T> {
        type T = T;
        type TX = Arc<T>;
        type IM = ArcSwap<T>;
    }

    impl<T> Cfg<RefCellRcP<T>> {
        pub fn new(refresh: fn() -> T) -> Self {
            let cfg = Self {
                cache: RefCell::new(Rc::new(refresh())),
                refresh,
            };
            cfg
        }
    }

    impl<T> Cfg<ArcSwapArcP<T>> {
        pub fn new(refresh: fn() -> T) -> Self {
            let cfg = Self {
                cache: ArcSwapAny::new(Arc::new(refresh())),
                refresh,
            };
            cfg
        }
    }

    pub type CfgRefCellRc<T> = Cfg<RefCellRcP<T>>;

    pub type CfgArcSwapArc<T> = Cfg<ArcSwapArcP<T>>;
}

pub fn packing_main() {
    fn refresh() -> i32 {
        static REFRESH_VALUE: AtomicI32 = AtomicI32::new(1);
        REFRESH_VALUE.fetch_add(1, Ordering::Relaxed)
    }

    println!("packing_main");

    {
        use a::*;

        println!("submodule a");

        {
            let cfg = CfgRefCellRc::new(refresh);
            println!("{}", cfg.get());
            cfg.refresh();
            println!("{}", cfg.get());
        }

        {
            let cfg = CfgArcSwapArc::new(refresh);
            println!("{}", cfg.get());
            cfg.refresh();
            println!("{}", cfg.get());
        }
    }

    {
        use b::*;

        println!("submodule b");

        {
            let cfg = Cfg::<RefCellRcP<_>>::new(refresh);
            println!("{}", cfg.get());
            cfg.refresh();
            println!("{}", cfg.get());
        }

        {
            let cfg = Cfg::<ArcSwapArcP<_>>::new(refresh);
            println!("{}", cfg.get());
            cfg.refresh();
            println!("{}", cfg.get());
        }

        {
            let cfg = CfgRefCellRc::new(refresh);
            println!("{}", cfg.get());
            cfg.refresh();
            println!("{}", cfg.get());
        }

        {
            let cfg = CfgArcSwapArc::new(refresh);
            println!("{}", cfg.get());
            cfg.refresh();
            println!("{}", cfg.get());
        }
    }
}
