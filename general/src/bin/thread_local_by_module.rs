fn main() {
    m1::FOO.with(|x| println!("m1::FOO={}", x));
    m2::FOO.with(|x| println!("m2::FOO={}", x));
}

mod m1 {
    thread_local! {
        pub static FOO: u32 = 0;
    }
}

mod m2 {
    thread_local! {
        pub static FOO: i64 = 1;
    }
}
