// 泛型


fn largest_test(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 泛型
// fn largest<T>(list: &[T]) -> T {
    // let mut largest = list[0];

    // // for &item in list.iter() {
        // // if item > largest {
            // // largest = item;
        // // }
    // // }

    // largest
// }

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_test(&numbers);
    assert_eq!(result, 100);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_test(&numbers);
    assert_eq!(result, 6000);

    let integer = Point{x: 1, y :2};
    println!("Integer point: {:?}", integer.x);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point1<T> {
    x: T,
    y: T,
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
