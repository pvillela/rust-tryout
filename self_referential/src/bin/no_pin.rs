#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(tat: &str) -> Self {
        Test {
            a: String::from(tat),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*(self.b) }
    }

    fn bp(&self) -> *const String {
        self.b
    }
}

fn main() {
    let mut var1 = Test::new("test1");
    var1.init();

    let mut var2 = Test::new("test2");
    var2.init();

    println!("-- before swap");

    println!(
        "address of var1: {:p}, var1.a(): {}, var1.b(): {}, var1.bp(): {:?}",
        &var1,
        var1.a(),
        var1.b(),
        var1.bp()
    );
    println!(
        "address of var2: {:p}, var2.a(): {}, var2.b(): {}, var2.bp(): {:?}",
        &var2,
        var2.a(),
        var2.b(),
        var2.bp()
    );

    std::mem::swap(&mut var1, &mut var2);

    println!("-- after swap");

    println!(
        "address of var1: {:p}, var1.a(): {}, var1.b(): {}, var1.bp(): {:?}",
        &var1,
        var1.a(),
        var1.b(),
        var1.bp()
    );
    println!(
        "address of var2: {:p}, var2.a(): {}, var2.b(): {}, var2.bp(): {:?}",
        &var2,
        var2.a(),
        var2.b(),
        var2.bp()
    );
}
