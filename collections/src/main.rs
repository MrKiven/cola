
fn another() {
    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    assert_eq!(vec1, [1, 2, 3, 4]);
}

fn stack() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

#[warn(unused_variables)]
fn string_about() {
    println!("\t\n");
    let s1 = String::new();
    println!("s1: {:?}", s1);
    let data = "Initial contents";
    let s1 = data.to_string();
    let s3 = "Initial contents".to_string();
    println!("s3: {:?}", s3);
    let s4 = String::from("hello world");
    println!("s4: {:?}", s4);
    let mut s = String::from("Hello");
    s.push_str(" World!");
    assert_eq!(s, String::from("Hello World!"));
    println!("Your string is {:?}", s);

    assert_eq!(add_string(s1, " success."), "Initial contents success.");
    println!("Add two string: 'hello' + 'world' = '{}'", add_string("hello".to_string(), " world"));

    let my_str = String::from("Hello Kiven!");

    for c in my_str.chars() {
        println!("{}", c);
    }

    let s = &my_str[0..5];
    println!("my_str is: {}", my_str);
    assert_eq!(s, "Hello");
    println!("&my_str[0..5] is: {}", s);
}

fn add_string(s1: String, s2: &str) -> String {
    s1 + s2
}


fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3].iter().cloned());

    for x in &vec {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    another();

    stack();

    string_about();

    map_about();
}


fn map_about() {
    // import
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Kiven"), 100);
    scores.insert(String::from("Jack"), 90);
    println!("socres is: {:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 枚举
    scores.insert(String::from("Blue"), 80);
    let hello = String::from("Hello");
    scores.entry(hello).or_insert(10);
    println!("socres is: {:?}", scores);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
