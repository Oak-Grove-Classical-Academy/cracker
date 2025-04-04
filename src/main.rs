use rand::Rng;
use rand::distr::Alphanumeric;
use std::env;

fn main() {
    // println!("Password: dolphins");
    // The prelude import enables methods we use below, specifically
    // Rng::random, Rng::sample, SliceRandom::shuffle and IndexedRandom::choose.

    // // Get an RNG:
    // let mut rng = rand::rng();
    //
    // // Try printing a random alphanumeric value instead!
    // println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);

    // Prints each argument on a separate line
    let arg: Vec<String> = env::args().collect();

    let password_length = arg[1].parse::<i32>().unwrap();

    let mut rng = rand::rng();
    let chars: String = (0..password_length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    println!("Random chars: {}", chars);
}
