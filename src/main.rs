mod engine;
mod utils;
use engine::Engine;
use utils::{bytes_to_i64, bytes_to_str, i64_to_bytes, str_to_bytes};

fn main() {
    let mut engine = Engine::new();
    let key = "example";
    // let value = 1234.to_string().into_bytes();
    let value = i64_to_bytes(1234);
    engine.set(key, value.clone());
    // Convert the retrieved bytes into an integer
    let retreived_value = bytes_to_i64(engine.get(key).unwrap());
    println!("{:?}", retreived_value);

    let value = str_to_bytes("Hello, world!");
    engine.set(key, value.clone());

    let retreived_value = bytes_to_str(engine.get(key).unwrap());
    println!("{:?}", retreived_value);

    println!("{:?}", engine.delete(key));
}
