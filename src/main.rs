use kupyna_512;

fn main() {
    let message = b"".to_vec();
    let _message_length = 0;

    let hash_code = kupyna_512::hash(message, None).unwrap();

    println!("{:02X?}", hash_code);
}