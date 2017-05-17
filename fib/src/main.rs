
fn fib(num: i32) -> i64 {
    if num < 0 {
        panic!("{} is negative!", num)
    }

    match num {
        0     => panic!("zero is not a right argument to fibonacci!"),
        1 | 2 => 1,
        3     => 2,
        /*
        * 50  => 12586269025,
        * */
        _     => fib(num - 1) + fib(num - 2)
    }
}

fn main() {
    let num: i32 = 10;
    let result = fib(num);
    println!("result of `fib({})` is {}", num, result)
}
