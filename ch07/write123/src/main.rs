use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = Vec::new();

    let one = 1_u32;
    let two = 2_i8;
    let three = 3.0_f64;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", w);

    w.write_i8(two).unwrap();
    println!("{:?}", w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one = r.read_u32::<LittleEndian>().unwrap();
    let two = r.read_i8().unwrap();
    let three = r.read_f64::<LittleEndian>().unwrap();

    (one, two, three)
}

fn main() {
    let (one, two, three) = write_numbers_to_file();
    let (one_, two_, three_) = read_numbers_from_file();

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}
