/// Used to convert a reference to another type with the same lifetie.
pub trait RefInto<'a, U> {
    fn ref_into(&'a self) -> U
    where
        U: 'a;
}

/// Used to construct the target type from a reference to the source type.
pub trait Make<T> {
    fn make(&self) -> T;
}

impl<T, F> Make<T> for F
where
    F: Fn() -> T,
{
    fn make(&self) -> T {
        self()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[allow(unused)]
    struct AppCfgInfo0 {
        pub x: String,
        pub y: i32,
        pub z: bool,
    }

    type AppCfgInfo = Arc<AppCfgInfo0>;

    fn cfg_src() -> AppCfgInfo {
        AppCfgInfo0 {
            x: "initial".to_owned(),
            y: 42,
            z: false,
        }
        .into()
    }

    #[derive(PartialEq, Debug)]
    struct MyCfgInfo<'a> {
        pub u: i32,
        pub v: &'a str,
    }

    impl<'a> RefInto<'a, MyCfgInfo<'a>> for AppCfgInfo {
        fn ref_into(&'a self) -> MyCfgInfo<'a> {
            MyCfgInfo {
                u: self.y,
                v: &self.x,
            }
        }
    }

    fn foo(cfg_src: impl Make<AppCfgInfo>) {
        let app_cfg_info = cfg_src.make();
        let my_cfg = app_cfg_info.ref_into();
        assert_eq!(
            my_cfg,
            MyCfgInfo {
                u: 42,
                v: "initial"
            }
        );
    }

    #[test]
    fn test() {
        foo(cfg_src);
    }
}
