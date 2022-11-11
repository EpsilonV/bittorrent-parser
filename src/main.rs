pub enum Bencode {
    Int(i64),

    Bytes(Vec<u8>),

    List(Vec<Self>),

    Dict(std::collections::BTreeMap<Vec<u8>, Self>),
}
fn main() {
    println!("Hello, world!");
    let s = "123".to_string();
    s.parse::<i64>().map(Bencode::Int);
    let i = Bencode::int(10);

    print!("fdasf {}", i);
}
