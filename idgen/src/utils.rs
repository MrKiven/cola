extern crate hostname;
extern crate libc;

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
