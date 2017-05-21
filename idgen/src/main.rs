mod utils;
extern crate ascii;

use ascii::ToAsciiChar;

use utils::gethostname;
use utils::to_ascii_number;



fn main () {
    println!("Current hostname: {:?}", gethostname());


    println!("'A' to ascii: {:?}", match 'a'.to_ascii_char() {
        Ok(ascii) => ascii,
        Err(_)    => panic!("Unknown"),
    });

    println!("{}", to_ascii_number('a'));
}
