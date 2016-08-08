use std::env;
use std::io::{self, Write};
use std::process;
use std::borrow::Cow;

#[cfg(not(unix))]
mod platform {
    use std::ffi::OsString;
    pub const BUFFER_CAPACITY: usize = 16 * 1024;

    pub fn to_bytes(os_str: OsString) -> Vec<u8> {
        os_str.into_string().expect("non utf-8 argument only supported on unix").into()
    }
}

#[cfg(unix)]
mod platform {
    use std::ffi::OsString;
    pub const BUFFER_CAPACITY: usize = 64 * 1024;

    pub fn to_bytes(os_str: OsString) -> Vec<u8> {
        use std::os::unix::ffi::OsStringExt;
        os_str.into_vec()
    }
}

use platform::*;

fn fill_up_buffer<'a>(buffer: &'a mut [u8], output: &'a [u8]) -> &'a [u8] {
    if output.len() > buffer.len() / 2 {
        return output;
    }

    let mut buffer_size = output.len();
    buffer[..buffer_size].clone_from_slice(output);

    while buffer_size < buffer.len() / 2 {
        let (left, mut right) = buffer.split_at_mut(buffer_size);
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
    write(&env::args_os()
        .nth(1)
        .map(to_bytes)
        .map_or(Cow::Borrowed(&b"y\n"[..]), |mut arg| {
            arg.push(b'\n');
            Cow::Owned(arg)
        }));
    process::exit(1);
}
