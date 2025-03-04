use dns_parse::decode_dns_name;

fn main() {
    let pkt = b"\x06google\x03com\0";

    println!(
        "{}",
        std::str::from_utf8(decode_dns_name(&pkt[..]).as_ref().unwrap()).unwrap()
    );
}
