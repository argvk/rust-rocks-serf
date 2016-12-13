extern crate md5;
extern crate byteorder;

use self::byteorder::{BigEndian, ByteOrder};

pub fn rendezvous(nodes: Vec<&str>, key: &str) -> String {
    let mut node_in = nodes[0];
    let mut max_val = 0;
    for node in &nodes {
        let dig = md5::compute(format!("{}{}",node, key).as_bytes());
        let dig_long = BigEndian::read_i32(&dig[..]).abs();

        if dig_long > max_val {
            node_in = node.clone();
            max_val = dig_long.clone();
        }

        print!("dig {} ", dig_long);
        println!("node : {}, str: {}", node, key);
    }
    node_in.to_string()
}
