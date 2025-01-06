use std::{thread, time};

fn main() {
    (1..1001).for_each(|n| {
        let start = time::Instant::now();
        let handlers = (0..n)
            .map(|_m| {
                thread::spawn(|| {
                    let start = time::Instant::now();
                    let pause = time::Duration::from_millis(20);
                    while start.elapsed() < pause {
                        thread::yield_now();
                    }
                })
            })
            .collect::<Vec<_>>();

        handlers
            .into_iter()
            .for_each(|handle| handle.join().unwrap());

        let finish = time::Instant::now();
        println!("{}\t{:?}", n, finish.duration_since(start));
    });
}
