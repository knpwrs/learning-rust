// Message passing: Do not communicate by sharing memory; instead, share memory by communicating.
use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        // println!("The value is: {}", val); // We can't do this. We no longer own the value.
    });
    // Blocks waiting for a message
    let received = rx.recv().unwrap();
    println!("Received {} from spawned thread!", received);

    // Another example
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("foo"),
            String::from("bar"),
            String::from("baz"),
            String::from("bananas"),
            String::from("bears"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for message in rx {
        println!("Received: {}", message);
    }

    // Cloning the transmitter (*multiple* producer)
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("foo"),
            String::from("bar"),
            String::from("baz"),
            String::from("bananas"),
            String::from("bears"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Do not clone again otherwise the main thread will block
    // let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("HELLO"),
            String::from("WOLRD"),
            String::from("FOO"),
            String::from("BAR"),
            String::from("BAZ"),
            String::from("BANANAS"),
            String::from("BEARS"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for message in rx {
        println!("Received: {}", message);
    }
}
