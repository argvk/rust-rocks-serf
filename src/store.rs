extern crate rocksdb;
use rocksdb::{DB};

// TODO make this a trait
pub fn get(key: &str) -> Result<String, String> {
    let db = DB::open_default("storage").unwrap();
    match db.get(key.as_bytes()) {
        Ok(Some(value)) => {
            return Ok(value.to_utf8().unwrap().to_string());
        },
        Ok(None) => return Ok(String::from("")),
        Err(e) =>
            {
                println!("error while trying to get object from db {}", e);
                return Err(format!("error while trying to fetch info {:?}", e));
            },
    };
}

pub fn put(key: &str, val:&str) -> Result<String, String> {
    let db = DB::open_default("storage").unwrap();
    match db.put(key.as_bytes(), val.as_bytes()){
        Ok(_) => return Ok(String::from("added")),
        Err(e) => {
            println!("error while trying to put object into db {}", e);
            return Err(format!(""));
        },
    }
}

pub fn delete(key: &str) -> Result<String, String> {
    let db = DB::open_default("storage").unwrap();
    match db.delete(key.as_bytes()){
        Ok(_) => return Ok(String::from("deleted")),
        Err(e) => {
            println!("error while trying to delete object into db {}", e);
            return Err(format!(""));
        },
    }
}
