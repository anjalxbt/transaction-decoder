#[allow(unused_variables)]
fn read_version(transaction_hex:&str)->u32{
    return 1;
}

fn main() {
    let version = read_version("adhfaweorhawer");
    println!("version: {}!", version);
}
