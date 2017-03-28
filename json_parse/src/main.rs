extern crate rustc_serialize;
// 引入rustc_serialize模块
use rustc_serialize::json;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
// 定义TestStruct
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

fn main() {
    // 初始化TestStruct
    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };

    // Serialize using `json::encode`
    // 将TestStruct转意为字符串
    let encoded = json::encode(&object).unwrap();
    println!("{}",encoded);
    // Deserialize using `json::decode`
    // 将json字符串中的数据转化成TestStruct对应的数据，相当于初始化
    let decoded: TestStruct = json::decode(&encoded).unwrap();
    println!("{:?}",decoded.data_vector);
}
