use std::env;
use std::io::{self, Write};
use std::process;

pub const BUFFER_CAPACITY: usize = 64 * 1024;

fn fill_up_buffer<'a>(buffer: &'a mut [u8], output: &'a [u8]) -> &'a [u8] {
    if output.len() > buffer.len() / 2 {
        return output;
    }

    let mut buffer_size = output.len();
    buffer[..buffer_size].clone_from_slice(output);

    while buffer_size < buffer.len() / 2 {
        let (left, right) = buffer.split_at_mut(buffer_size);
        right[..buffer_size].clone_from_slice(left);
        buffer_size *= 2;
    }

    &buffer[..buffer_size]
}


fn write(output: &[u8]) {
    let stdout = io::stdout();
    let mut locked = stdout.lock();
    let mut buffer = [0u8; BUFFER_CAPACITY];

    let filled = fill_up_buffer(&mut buffer, output);
    while locked.write_all(filled).is_ok() {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let string = if args.is_empty() {
        "y\n".to_owned()
    } else {
        args[1..].join(" ") + "\n"
    };    
    
    write(&string.as_bytes());
    
    process::exit(1);
}
