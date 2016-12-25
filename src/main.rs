mod store;
mod hash;
extern crate mpack;
extern crate rocksdb;

use mpack::rpc::{Client};
use mpack::{Value, write};
use std::net::TcpStream;

fn main() {

    let mut serf_endpoint = TcpStream::connect("127.0.0.1:31789").unwrap();

    write(&mut serf_endpoint, "{\"Command\": \"handshake\", \"Seq\": 0}".to_string()).unwrap();
    write(&mut serf_endpoint, "{\"Version\": 1}".to_string()).unwrap();

//    match serf_endpoint.try_clone() {
//        Ok(tcp_new_serf) => {
//            let mut cl = Client::new(tcp_new_serf, serf_endpoint);
//            let resp_res = cl.call_sync("".to_string(), vec![
//            Value::String("{\"Command\": \"handshake\", \"Seq\": 0}".to_string()),
//            Value::String("{\"Version\": 1}".to_string())
//            ]);
//            match resp_res {
//                Ok(val) => match val {
//                    Ok(resp_str) => println!("got a resp from bang {}", resp_str.string().unwrap()),
//                    Err(resp_str) => println!("got an error from bang {}", resp_str.string().unwrap())
//                },
//                Err(val) => println!("error while sending to bang ")
//            }
//        },
//        Err(e) => println!("error parsing header: {:?}", e),
//    }

    //     let mut conn = TcpStream::connect("127.0.0.1:31789").unwrap();

     // write values
//     write(&mut conn, 3 as i32).unwrap();

    let k = "bazinga";
    let v = "bv";
    let node = hash::rendezvous(vec!["10.0.0.20", "10.0.2.1"], k);
    println!("must be in {}", node);
    match store::put(k, v) {
        Ok(_) => println!("added {}=>{}", k, v),
        Err(e) => println!("error {}", e),
    }
    match store::get(k) {
        Ok(val) => println!("got {}", val),
        Err(e) => println!("error {}", e),
    }
    match store::delete(k) {
        Ok(_) => println!("deleted {}", k),
        Err(e) => println!("error {}", e),
    }
    match store::get(k) {
        Ok(val) => println!("got {}", val),
        Err(e) => println!("error {}", e),
    }
}
