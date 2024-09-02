use rand::{Rng, SeedableRng};

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let x: [f32; 64] = rng.gen();
    println!("Random f32: {:?}", x);
}
