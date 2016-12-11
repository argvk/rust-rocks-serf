extern crate md5;
extern crate byteorder;

use std::io::Cursor;
use self::byteorder::{BigEndian, ReadBytesExt, ByteOrder};

pub fn rendezvous(nodes: Vec<&str>, key: &str) -> String {
    let mut nodeIn = nodes[0];
    let mut maxVal = 0;
    for node in &nodes {
        let dig = md5::compute(format!("{}{}",node, key).as_bytes());
        let digLong = BigEndian::read_i32(&dig[..]).abs();

        if digLong > maxVal {
            nodeIn = node.clone();
            maxVal = digLong.clone();
        }

        print!("dig {} ", digLong);
        println!("node : {}, str: {}", node, key);
    }
    nodeIn.to_string()
}
