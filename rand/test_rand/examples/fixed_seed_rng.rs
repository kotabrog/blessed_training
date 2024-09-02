use rand::{Rng, SeedableRng};

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    println!("Random f32: {}", rng.gen::<f32>());
}
