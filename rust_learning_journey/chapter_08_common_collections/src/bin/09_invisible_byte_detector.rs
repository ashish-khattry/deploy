//09_invisible_byte_detector
fn main() {
    for data in 1..=31 as u8 {
        let letter = data as char;
        let real_letter = letter.escape_debug();
        println!("Letter={} byte={}", real_letter, data);
    }
}
