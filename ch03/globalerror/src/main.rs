use rand::random;

static mut ERROR: isize = 0;

struct File;

fn read(_f: &File, _save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }

    0
}

fn main() {
    let f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}
