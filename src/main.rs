mod engine;
use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    let key = "example";
    let value = 1234.to_string().into_bytes();

    engine.set(key, value.clone());

    // Convert the retrieved bytes into an integer
    let retreived_value = String::from_utf8(engine.get(key).unwrap()).unwrap();
    println!("{:?}", retreived_value);
}
