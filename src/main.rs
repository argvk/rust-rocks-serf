mod store;
extern crate rocksdb;

fn main() {

    let k = "bazinga";
    let v = "bv";
    store::put(k, v);
    match store::get(k) {
        Ok(val) => println!("got {}", val),
        Err(e) => println!("error {}", e),
    }
    store::delete(k);
}
