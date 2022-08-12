use std::{thread, time::Duration};
// multiple producer, single consumer
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    // INFO: spawn() takes a closure
    // => similar to callbacks
    let to100 = thread::spawn(|| {
        for x in 0..20 {
            println!("spawned thread {}", x);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // join() blocks the thread queue and forces , to placing to100.join()
    // here whould blocks the main thread from executing
    for y in 0..10 {
        println!("main thread {}", y);
        thread::sleep(Duration::from_millis(50));
    }
    // here would fire both threads cause the main code is already compiled
    to100.join().unwrap();

    let v = vec![1, 2, 3];
    // force a move of data ownership here with the move keyword
    // otherwise v can get dropped before join()
    let handle = thread::spawn(move || {
        println!("vector {:?}", v);
    });
    handle.join().unwrap();

    // INFO: CHANNELS
    let (tx, rx) = mpsc::channel(); // returns transmitter and receiver tuple

    thread::spawn(move || {
        // needs move here because the spawned thread needs to own tx
        let var = String::from("hi");
        tx.send(var).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("from received channel: {}", received);

    let (tx, rx) = mpsc::channel();
    let tx_twin = tx.clone(); // multiple producers

    let list: Vec<String> = vec![
        String::from("list"),
        String::from("of"),
        String::from("item"),
    ];
    thread::spawn(move || {
        for item in list {
            tx_twin.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("abc"), String::from("123")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("received data {}", received);
    }
    println!("testing blocking"); // this is blocked by the received for loop

    // NOTE: SHARED STATE CONCURRENCY
    // mutex: mutual exclusion

    // NOTE: single threaded context
    let m = Mutex::new(5);
    { // put the lock inside a scope to represent its turn using the mutex
        let mut num = m.lock().unwrap();
        *num = 15;
    }
    println!("value of mutex m is {:?}", m);

    // NOTE: multithreaded context
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![]; // thread handles
    // this vector will always print out correctly in series because we enforced mutex into the concurrency
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("{}", num);
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result {}", *counter.lock().unwrap());
}
