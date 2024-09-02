use rand::prelude::*;
use rand::distributions::WeightedIndex;

fn main() {
    let choices = ['a', 'b', 'c', 'd'];
    let weights = [2, 1, 1, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let choice = choices[dist.sample(&mut rng)];
    println!("Random choice: {}", choice);

    let weights = [0.2, 0.3, 0.1, 0.4];
    let dist = WeightedIndex::new(&weights).unwrap();
    let choice = choices[dist.sample(&mut rng)];
    println!("Random choice: {}", choice);
}
