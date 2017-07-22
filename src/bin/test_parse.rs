extern crate ax25;

use ax25::frame::parse_from_raw;

fn main() {
    let packet: Vec<u8> = vec![0x00, 0x92, 0x88, 0x40, 0x40, 0x40, 0x40, 0xE0, 0xAC, 0x96, 0x6E, 0x90, 0x88, 0x9A, 0x6D, 0x03, 0xF0, 0x56, 0x4B, 0x37, 0x48, 0x44, 0x4D, 0x2D, 0x36, 0x20, 0x50, 0x61, 0x63, 0x6B, 0x65, 0x74, 0x20, 0x49, 0x6E, 0x74, 0x65, 0x72, 0x6E, 0x65, 0x74, 0x20, 0x47, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x20, 0x51, 0x45, 0x33, 0x37, 0x50, 0x47];

    println!("Parse result:");
    match parse_from_raw(packet) {
        Ok(parsed) => println!("{}", parsed),
        Err(e) => println!("Could not parse! {}", e)
    };
}
