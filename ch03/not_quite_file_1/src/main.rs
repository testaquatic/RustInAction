use not_quite_file_1::{close, open, File, Read};

fn main() {
    let mut f6 = File::new("5.txt");

    let mut buffer = Vec::new();

    if f6.read(&mut buffer).is_err() {
        println!("Error Checking is working");
    }

    f6 = open(f6).unwrap();
    let f5_length = f6.read(&mut buffer).unwrap();
    f6 = close(f6).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{}", f6);
    println!("{} is {} bytes long", f6.name(), f5_length);
    println!("{}", text);
}
