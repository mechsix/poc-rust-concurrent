use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};


fn calc_pi(samples: i32) -> f64 {
    let mut rnd = thread_rng();
    let mut counter = 0;

    for _i in 0..samples {
        let x = rnd.gen::<f64>();
        let y = rnd.gen::<f64>();
        if x.powi(2) + y.powi(2) < 1.0 {
            counter += 1;
        }
    }
    (counter as f64 / samples as f64) * 4.0
}

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
    println!("{}", calc_pi(1000000));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).expect("");

    println!("{:?}", end-start);
}
