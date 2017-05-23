extern crate hostname;
extern crate libc;
extern crate rand;

use self::rand::Rng;

pub fn gethostname() -> String {
    let name = hostname::get_hostname();
    match name {
        Some(name) => name,
        None       => String::from("Unknown hostname"),
    }
}

pub fn to_ascii_number(c: char) -> i8 {
    c as libc::c_char
}

pub fn range_number(from: i32, to: i32) -> i32 {
    rand::thread_rng().gen_range(from, to)
}
