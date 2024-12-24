fn parity_bit(bytes: &[u8]) -> u8 {
    let n_ones = bytes
        .iter()
        .map(|byte| {
            let ones = byte.count_ones();
            println!("{byte} ({byte:010b}) has {ones} one bits");
            ones
        })
        .sum::<u32>();

    (n_ones % 2 == 0) as u8
}

fn main() {
    let abc = b"abc";
    println!("input: {abc:?}");
    println!("output: {:08x}", parity_bit(abc));
    println!();
    let abcd = b"abcd";
    println!("input: {abcd:?}");
    println!("result: {:08x}", parity_bit(abcd));
}
