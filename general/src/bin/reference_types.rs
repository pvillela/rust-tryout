use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

fn f_ref<T: Debug>(x: &T) {
    println!("{:?}", x);
}

fn f_as_ref<T: Debug, U: AsRef<T> + Debug>(x: U) {
    let y = x.as_ref();
    println!("{:?}", y);
    f_ref(&x);
}

fn f_deref<T: Debug + Clone, U: Deref<Target = T> + Debug>(x: U) {
    let y = (*x).clone();
    println!("{:?}", y);
    f_ref(&x);
}

fn f_borrow<T: Debug + Clone, U: Borrow<T>>(x: U) {
    let y = (*x.borrow()).clone();
    println!("{:?}", y);
}

fn f_borrow_mut<T: Debug + Clone, U: BorrowMut<T>>(mut x1: U, x2: U) {
    let y = x1.borrow_mut();
    let z = x2.borrow();
    *y = z.clone();
    println!("{:?}", y);
}

fn f_ref_ref<T: Debug>(x: &T) {
    f_ref::<T>(x);
    f_ref::<T>(&x);
}

fn f_ref_arc<T: Debug>(x: Arc<T>) {
    f_ref(&x)
}

// None of the lines in fn below compile.
// fn f_as_ref_ref<T: Debug>(x: &T) {
//     f_as_ref(x);
//     f_as_ref(&x);
// }

fn f_as_ref_arc<T: Debug>(x: Arc<T>) {
    f_as_ref(x);
}

fn f_deref_arc<T: Debug + Clone>(x: Arc<T>) {
    f_deref(x);
}

fn f_borrow_arc<T: Debug + Clone>(x: Arc<T>) {
    f_borrow::<T, Arc<T>>(x);
}

// Does not compile because Arc<T> is not BorrowMut<T>.
// fn f_borrow_mut_arc<T: Debug + Clone>(mut x1: Arc<T>, x2: Arc<T>) {
//     f_borrow_mut::<T, Arc<T>>(x1, x2);
// }

fn f_ref_rc<T: Debug>(x: Rc<T>) {
    f_ref(&x)
}

fn f_as_ref_rc<T: Debug>(x: Rc<T>) {
    f_as_ref(x);
}

fn f_deref_rc<T: Debug + Clone>(x: Rc<T>) {
    f_deref(x);
}

fn f_borrow_rc<T: Debug + Clone>(x: Rc<T>) {
    f_borrow::<T, Rc<T>>(x);
}

// Does not compile because Arc<T> is not BorrowMut<T>.
// fn f_borrow_mut_rc<T: Debug + Clone>(mut x1: Rc<T>, x2: Rc<T>) {
//     f_borrow_mut::<T, Rc<T>>(x1, x2);
// }

fn main() {
    let x1 = "foo".to_owned();
    let x2 = "bar".to_owned();
    let a1 = Arc::new(x1.clone());
    let r1 = Rc::new(x1.clone());

    f_ref_ref(&x1);

    f_ref_arc(a1.clone());
    f_as_ref_arc(a1.clone());
    f_deref_arc(a1.clone());
    f_borrow_arc(a1);

    f_ref_rc(r1.clone());
    f_as_ref_rc(r1.clone());
    f_deref_rc(r1.clone());
    f_borrow_rc(r1);

    f_deref(&x1);
    f_borrow_mut::<String, String>(x1, x2);
}
