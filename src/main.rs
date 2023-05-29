mod engine;
use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    let key = "example";
    let value = vec![1, 2, 3];

    engine.set(key, value.clone());
    println!("{:?}", engine.get(key));
}
