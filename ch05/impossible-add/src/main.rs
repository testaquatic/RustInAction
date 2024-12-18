// 이 부분이 없으면 컴파일러에서 오류를 반환한다.
#![allow(arithmetic_overflow)]

fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    // cargo run => 패닉
    // cargo run --release => 200 + 200 = 144
    println!("200 + 200 = {c}");
}
