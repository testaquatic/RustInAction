fn main() {
    let a = 42.42_f32;

    // `unsafe`` 사용을 피했다.
    let frankentype = a.to_bits();
    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b = unsafe { std::mem::transmute::<_, f32>(frankentype) };
    println!("{}", b);

    assert_eq!(a, b);
}
