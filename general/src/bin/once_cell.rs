use once_cell::sync::OnceCell;
use std::{sync::Arc, thread::spawn};

#[allow(unused)]
#[derive(Debug, Clone)]
struct Info {
    x: String,
}

struct InfoSrc {
    src: Box<dyn 'static + Fn() -> Info + Send + Sync>,
}

impl InfoSrc {
    pub fn get(&self) -> Info {
        self.src.as_ref()()
    }
}

static INFO_SRC: OnceCell<Arc<InfoSrc>> = OnceCell::new();

fn main() {
    let info = Info {
        x: "foo".to_owned(),
    };

    if INFO_SRC
        .set(Arc::new(InfoSrc {
            src: Box::new(move || info.clone()),
        }))
        .is_err()
    {
        panic!("INFO_SRC already set")
    }

    let info_src = INFO_SRC.get().unwrap();

    let handle1 = spawn(move || {
        println!("info_src() from thread1: {:?}", (info_src.src)());
        println!("info_src() from thread1: {:?}", info_src.get());
    });

    println!("info_src() from main thread: {:?}", (info_src.src)());
    handle1.join().unwrap();
    println!("info_src() from main thread: {:?}", info_src.get());
}
