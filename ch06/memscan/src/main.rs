fn main() {
    let mut n_nonezero = 0;

    (1..10000)
        .map(|i| i as *const u8)
        .map(|ptr| unsafe { *ptr })
        .for_each(|byte_at_addr| {
            if byte_at_addr != 0 {
                n_nonezero += 1;
            }
        });

    println!("non-zero bytes in memory: {}", n_nonezero);
}
