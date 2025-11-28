use std::env::{Args, args};

pub fn get_args() -> Args {
    args()
}

pub fn get_args_str_buffer() -> Vec<String> {
    let raw_args = get_args();
    let mut buffer = Vec::<String>::new();
    for arg in raw_args {
        buffer.push(String::from(arg.as_str()));
    }
    buffer
}

pub struct ArgToken {}
