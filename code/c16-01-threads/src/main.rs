use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // The spawned thread will be terminated when the main thread exits, not when the spawned
    // thread exits! Let's make sure we wait for the spawned thread. `join` blocks the current
    // thread until the thread represented by the handle terminates.
    handle.join().unwrap();

    // Moving ownership to another thread
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's your vector, Victor: {:?}", v);
    });
    // drop(v); // can't do this, `v` was moved
    handle.join().unwrap();
}
