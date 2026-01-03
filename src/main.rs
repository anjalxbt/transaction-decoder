#[allow(unused_variables)]
fn read_version(transaction_hex:&str)->u32{
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = &transaction_bytes[0..4];
    println!("version bytes: {:?}", version_bytes);
    return 1;
}

fn main() {
    let version = read_version("0123456789");
    println!("version: {}!", version);
}
