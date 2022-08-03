use rand::{thread_rng, Rng};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Worklings {
    _id: String,
    content: String,
}

impl Worklings {
    fn new() -> Self {
        // vec from given [element; size]
        let mut rng = thread_rng();
        let mut chars: Vec<u8> = vec![0; 5];
        for code in &mut chars {
            // AZaz
            let mut ch = rng.gen_range(65..117);
            if ch > 90 {
                ch += 6;
            }
            *code = ch;
        }
        Worklings {
            _id: String::from_utf8(chars.clone()).unwrap(),
            content: String::from_utf8(chars).unwrap(),
        }
    }
}

fn work(workling: &mut Worklings) -> &mut Worklings {
    let mut rng = thread_rng();
    let mut chars: Vec<u8> = vec![0; 5];
    for code in &mut chars {
        // AZaz
        let ch = rng.gen_range(48..57);
        *code = ch;
    }
    workling.content = format!(
        "{}{}",
        &workling._id,
        String::from_utf8(chars.clone()).unwrap()
    );
    workling
}
fn main() {
    // INFO: thread 1 do a very long calculation
    // INFO: thread 2 do a very short calculation and share info with thread 3
    // INFO: thread 3 do a very long calculation but shorter than 1, shares
    // INFO: info with thread 2
    // INFO: sharing info algo: thread 2 will fill up with random strings (len
    // INFO: 5), thread 3 will append 5 random numbers to said data

    // NOTE: all threads should not block each other

    let mut lone_data: Vec<i32> = Vec::new();
    let shared_data: Vec<Worklings> = Vec::new();
    let mut id1 = 0;
    // INFO: read on Option
    // https://users.rust-lang.org/t/passing-mutable-struct-between-threads/28642/5
    let shared_data: Arc<Mutex<Vec<Worklings>>> = Arc::new(Mutex::new(shared_data));
    {
        // INFO: THREADING 1
        let handle1 = thread::spawn(move || loop {
            lone_data.push(id1);
            println!("[SLOW, SOLO] 1: {}", id1);
            thread::sleep(Duration::from_secs(2));
            id1 += 1;
        });

        // WARNING: IMPORTANT, clone needs to be outsite thread
        let shared_data = Arc::clone(&shared_data);
        let (tx, rx) = mpsc::channel();

        // INFO: THREADING 2
        let handle2 = thread::spawn(move || loop {
            let item = Worklings::new();
            println!("[FAST, SHARED] 2: {}", &item.content);

            let mut mutex_vec = shared_data.lock().unwrap();
            mutex_vec.push(item.clone());

            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(500));
        });

        let handle3 = thread::spawn(move || loop {
            let mut item = rx.recv().unwrap();
            let ptr = work(&mut item);
            println!("{:?}", *ptr);

            thread::sleep(Duration::from_millis(1500));
        });

        let handles = vec![handle1, handle2, handle3];
        for handle in handles {
            handle.join().unwrap();
        }
    }
    println!("{:?}", *shared_data.lock().unwrap());
}

#[cfg(test)]
mod test {}
