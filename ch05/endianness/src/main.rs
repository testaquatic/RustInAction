fn main() {
    let big_endian = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian = [0xDD, 0xCC, 0xBB, 0xAA];

    // `unsafe`사용을 피했다.
    let a = <i32>::from_ne_bytes(big_endian);
    let b = <i32>::from_ne_bytes(little_endian);

    println!("{a} vs {b}");
}
