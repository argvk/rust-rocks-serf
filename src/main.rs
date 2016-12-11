mod store;
mod hash;

extern crate rocksdb;

fn main() {
    let k = "bazinga";
    let v = "bv";
    let node = hash::rendezvous(vec!["10.0.0.20", "10.0.2.1"], k);
    println!("must be in {}", node);
    store::put(k, v);
    match store::get(k) {
        Ok(val) => println!("got {}", val),
        Err(e) => println!("error {}", e),
    }
    store::delete(k);
}
