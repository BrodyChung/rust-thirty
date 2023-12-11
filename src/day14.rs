
use std::time::{SystemTime};
use std::io::{Cursor, Write};

pub fn concat_buf() {
    let mut buf = [0; 1024];

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get timestamp")
        .as_secs();
        let time_buf = format!("[time: {}]", timestamp).as_bytes();

    let helo = "helo";

    let mut cursor = Cursor::new(&mut buf[..]);
    write!(cursor, "{}", helo).unwrap();

    cursor.set_position(2);
    write!(cursor, "[time: {}]", timestamp).unwrap();
    
    let len = cursor.position() as usize;
    println!("{}", std::str::from_utf8(&buf[..len]).unwrap());
}