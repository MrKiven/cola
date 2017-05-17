#[derive(Debug)]
struct Rectangle {
    length: u32,
    width:  u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

fn main() {
    let rect1 = Rectangle{length: 10, width: 20};
    // println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
