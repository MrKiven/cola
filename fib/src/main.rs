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

fn fib_loop(num: i32) -> i64 {
    match num {
        0     => panic!("zero is not a right argument to fibonacci!"),
        1 | 2 => 1,
        3     => 2,
        _     => {
            let mut a = 0;
            let mut b = 1;
            let mut sum = 0;
            for _ in 0..num - 1  {
                sum = a + b;
                a = b;
                b = sum;
            }
            sum
        }
    }
}

fn main() {
    let num: i32 = 10;

    let result = fib(num);
    let result_loop = fib_loop(num);
    println!("result of `fib({})` is {}", num, result);
    println!("result of `fib_loop({})` is {}", num, result_loop)
}
