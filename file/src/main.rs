use std::io;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::ErrorKind;

fn read_username_from_file() -> Result<String, io::Error> {
    // or: let fd = File::open("hello.txt")?;
    let fd = File::open("hello.txt");

    let mut fd = match fd {
        Ok(file) => file,
        Err(e)   => return Err(e),
    };

    let mut s = String::new();
    // or: fd.read_to_string(&mut s)?;
    // Ok(s)
    match fd.read_to_string(&mut s) {
        Ok(_)  => Ok(s),
        Err(e) => Err(e),
    }

    /*
     * NOTE: `?` 只能被用于返回 `Result` 的函数 !!!
     *
     * */
}

fn main() {
    let mut fd = match File::open("./src/hello.txt") {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match File::create("./src/hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:#?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:#?}", error)
        }
    };

    // XXX
    // 失败时 panic 的捷径: unwrap 和 expect
    // let fd = File::open("hello.txt").unwrap();
    // let fd = File::open("hello.txt").expect("Failed to open hello.txt");



    /*
     * TODO
     * */
    println!("fd: {:#?}", fd);
    let mut contents_o = String::new();
    fd.read_to_string(&mut contents_o);
    println!("contents: {:#?}", contents_o);

    // seek to start
    File::seek(&mut fd, SeekFrom::Start(0));
    let mut buffer = BufReader::new(fd);
    let mut contents = String::new();
    buffer.read_to_string(&mut contents);
    assert_eq!(contents, "Hello World!\n");
    println!("contents: {:#?}", contents);

    let name = read_username_from_file().expect("Unknow exception");
    println!("Got user name: {}", name);
}
