use std::{
    collections::HashMap,
    mem,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    task::Waker,
    thread::{self, JoinHandle},
    time::Duration,
};

// The different states a task can have in this Reactor
pub enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

// This is a "fake" reactor. It does no real I/O, but that also makes our
// code possible to run in the book and in the playground
pub struct Reactor {
    // we need some way of registering a Task with the reactor. Normally this
    // would be an "interest" in an I/O event
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,

    // This is a list of tasks
    pub tasks: HashMap<usize, TaskState>,
}

// This represents the Events we can send to our reactor thread. In this
// example it's only a Timeout or a Close event.
#[derive(Debug)]
enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {
    // We choose to return an atomic reference counted, mutex protected, heap
    // allocated `Reactor`. Just to make it easy to explain... No, the reason
    // we do this is:
    //
    // 1. We know that only thread-safe reactors will be created.
    // 2. By heap allocating it we can obtain a reference to a stable address
    // that's not dependent on the stack frame of the function that called `new`
    pub fn new() -> Arc<Mutex<Self>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::new(),
        }));

        // Notice that we'll need to use `weak` reference here. If we don't,
        // our `Reactor` will not get `dropped` when our main thread is finished
        // since we're holding internal references to it.

        // Since we're collecting all `JoinHandles` from the threads we spawn
        // and make sure to join them we know that `Reactor` will be alive
        // longer than any reference held by the threads we spawn here.
        let reactor_clone = Arc::downgrade(&reactor);

        // This will be our Reactor-thread. The Reactor-thread will in our case
        // just spawn new threads which will serve as timers for us.
        let handle = thread::spawn(move || {
            let mut handles = vec![];

            // This simulates some I/O resource
            for event in rx {
                println!("REACTOR: {:?}", event);
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {
                        // We spawn a new thread that will serve as a timer
                        // and will call `wake` on the correct `Waker` once
                        // it's done.
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }

            // This is important for us since we need to know that these
            // threads don't live longer than our Reactor-thread. Our
            // Reactor-thread will be joined when `Reactor` gets dropped.
            handles
                .into_iter()
                .for_each(|handle| handle.join().unwrap());
        });
        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    // The wake function will call wake on the waker for the task with the
    // corresponding id.
    pub fn wake(&mut self, id: usize) {
        self.tasks
            .get_mut(&id)
            .map(|state| {
                // No matter what state the task was in we can safely set it
                // to ready at this point. This lets us get ownership over the
                // the data that was there before we replaced it.
                match mem::replace(state, TaskState::Ready) {
                    TaskState::NotReady(waker) => waker.wake(),
                    TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
                    _ => unreachable!(),
                }
            })
            .unwrap();
    }

    // Register a new task with the reactor. In this particular example
    // we panic if a task with the same id get's registered twice
    pub fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    // We simply checks if a task with this id is in the state `TaskState::Ready`
    pub fn is_ready(&self, id: usize) -> bool {
        self.tasks
            .get(&id)
            .map(|state| match state {
                TaskState::Ready => true,
                _ => false,
            })
            .unwrap_or(false)
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        // We send a close event to the reactor so it closes down our reactor-thread.
        // If we don't do that we'll end up waiting forever for new events.
        self.dispatcher.send(Event::Close).unwrap();
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}
