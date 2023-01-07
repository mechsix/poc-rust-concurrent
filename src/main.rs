use std::borrow::Borrow;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::sync::mpsc;
use rand::rngs::ThreadRng;

const SAMPLES: i32 = 10000000;
const THREADS: i32 = 5;

fn sample_pi() -> f64 {
    let mut rnd = thread_rng();
    let x = rnd.gen::<f64>();
    let y = rnd.gen::<f64>();
    if x.powi(2) + y.powi(2) < 1.0 {
        return 1.0
    }
    0.0
}

fn calc_pi_loop() -> f64 {
    let mut sum: f64 = 0.0;
    for _i in 0..SAMPLES {
        sum += sample_pi();
    }
    (sum / SAMPLES as f64) * 4.0
}

fn calc_pi_thread() -> f64 {
    let (tx, rx) = mpsc::channel();
    const SAMPLES_PER_THREAD: i32 = SAMPLES / THREADS;

    for t in 0..THREADS {
        let sender = tx.clone();
        thread::spawn(move || {
            let mut sum: f64 = 0.0;
            for _i in 0..SAMPLES_PER_THREAD {
                sum += sample_pi();
            }
            let result = (sum / SAMPLES_PER_THREAD as f64) * 4.0;
            println!("Thread{}: {}", t, result);
            sender.send(result).unwrap();
            drop(sender);
        });
    }
    drop(tx);

    let mut total = 0.0;
    for received in rx {
        total += received;
    }
    total / THREADS as f64
}

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
    // let pi = calc_pi_loop();
    let pi = calc_pi_thread();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).expect("");

    println!(
        "Samples {}\nDuration: {:?}\nPi: {}",
        SAMPLES,
        end-start,
        pi
    )
}
