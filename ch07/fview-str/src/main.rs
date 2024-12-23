use std::{env, fs::File, io::Read};

const BYTES_PER_LINE: usize = 16;

fn main() -> Result<(), std::io::Error> {
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");

    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while f.read_exact(&mut buffer).is_ok() {
        print!("[{:#010x?}] ", pos);
        buffer.iter().for_each(|byte| match byte {
            0x00 => print!(".  "),
            0xff => print!("## "),
            _ => print!("{byte:02x} "),
        });
        println!();
        pos += BYTES_PER_LINE;
    }

    Ok(())
}
