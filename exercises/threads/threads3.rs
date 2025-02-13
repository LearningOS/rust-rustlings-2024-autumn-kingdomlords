// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
    let qc1 = Arc::new(q);
    let qc2 = Arc::clone(&qc1);
   
    let handle1=thread::spawn({
        let tx = Arc::clone(&tx);
        move || {
            for val in &qc1.first_half {
                println!("sending {:?}", val);
                let _ =tx.lock().unwrap().send(*val);
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    let handle2=thread::spawn({
        let tx = Arc::clone(&tx);
        move || {
            for val in &qc2.second_half {
                println!("sending {:?}", val);
                let _ = tx.lock().unwrap().send(*val);
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    (handle1,handle2)
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;
    let tx = Arc::new(Mutex::new(tx));
    let(handle1,handle2)=send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
