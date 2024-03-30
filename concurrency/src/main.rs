use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let outer_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} hi number  from the spawned thread!", i);
            //  thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} main-hi number from the main thread!", i);
        //thread::sleep(Duration::from_millis(1));
        let inner_handle = thread::spawn(|| {
            for i in 1..10 {
                println!("{} another thread-hi number from the spawned thread!", i);
                //thread::sleep(Duration::from_millis(10));
            }
        });
        inner_handle.join().unwrap();
    }
    outer_handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    //Mutex - shared state concurrency
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
            println!("Counter: {}", *num);
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
