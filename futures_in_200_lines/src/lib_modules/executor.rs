use crate::{mywaker_into_waker, MyWaker};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    thread,
};

// Our executor takes any object which implements the `Future` trait
pub fn block_on<F: Future>(mut future: F) -> F::Output {
    println!("block_on called.");
    // the first thing we do is to construct a `Waker` which we'll pass on to
    // the `reactor` so it can wake us up when an event is ready.
    let mywaker = Arc::new(MyWaker {
        thread: thread::current(),
    });
    let waker = mywaker_into_waker(Arc::into_raw(mywaker));

    // The context struct is just a wrapper for a `Waker` object. Maybe in the
    // future this will do more, but right now it's just a wrapper.
    let mut cx = Context::from_waker(&waker);

    // So, since we run this on one thread and run one future to completion
    // we can pin the `Future` to the stack. This is unsafe, but saves an
    // allocation. We could `Box::pin` it too if we wanted. This is however
    // safe since we shadow `future` so it can't be accessed again and will
    // not move until it's dropped.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // We poll in a loop, but it's not a busy loop. It will only run when
    // an event occurs, or a thread has a "spurious wakeup" (an unexpected wakeup
    // that can happen for no good reason).
    let val = loop {
        match Future::poll(future.as_mut(), &mut cx) {
            // when the Future is ready we're finished
            Poll::Ready(val) => break val,

            // If we get a `pending` future we just go to sleep...
            Poll::Pending => thread::park(),
        };
    };
    val
}
