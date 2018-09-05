// Mutex: mutual exclusion
// Management of mutexes can be incredibly tricky to get right, which is why so many people are
// enthusiastic about channels. However, thanks to Rust’s type system and ownership rules, you
// can’t get locking and unlocking wrong.

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let m = Mutex::new(5);
    {
        // The call to lock would fail if another thread holding the lock panicked. In that case,
        // no one would ever be able to get the lock, so we’ve chosen to unwrap and have this
        // thread panic if we’re in that situation.
        let mut num = m.lock().unwrap();
        *num = 6;
        // The `Result` of `lock` is a `MutexGuard` which is a smart pointer which implements the
        // `Drop` trait to unlock the `Mutex` when it leaves scope.
    }
    println!("m = {:?}", m);

    // Real example:
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {:?}", *counter.lock().unwrap());

    // Using this strategy, you can divide a calculation into independent parts, split those parts
    // across threads, and then use a Mutex<T> to have each thread update the final result with its
    // part.
    //
    // In the same way we used RefCell<T> in to allow us to mutate contents inside an Rc<T>, we use
    // Mutex<T> to mutate contents inside an Arc<T>.
    //
    // Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the
    // thread-safe performance penalty.
    //
    // Beware of causing deadlocks! Rust cannot protect against them!
}
