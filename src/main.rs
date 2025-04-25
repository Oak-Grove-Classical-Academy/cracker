#![allow(unused_imports)]
use argon2::{
    Algorithm, Argon2, Params, ParamsBuilder, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use rand::Rng;
use rand::distr::Alphanumeric;
use rand::seq::IteratorRandom;
use std::env;

fn main() {
    // get all the command line arguments. The 0th element is the app name, 1st will be the
    // command, 2nd the length.
    let arg: Vec<String> = env::args().collect();

    // if there are no additional args given, the length of arg will be 1 (for the program name). Print
    // a nicer message if so.
    if arg.len() <= 1 {
        println!("Please provide a command.");
        return;
    }

    // get the first argument, command
    let command = arg[1].clone();

    // figure out which command the user chose, and run the respective function.
    if command == "random" {
        // get the second argument, length. We parse it into a usize type, which is just an unsigned
        // number.
        let password_length = arg[2].parse::<usize>().unwrap();
        random(password_length);
    } else if command == "phrase" {
        // get the second argument, length. We parse it into a usize type, which is just an unsigned
        // number.
        let password_length = arg[2].parse::<usize>().unwrap();

        phrase(password_length);
    } else if command == "md5" {
        // the second argument is the String to hash
        let password = &arg[2];

        println!("{:x}", md5::compute(password));
    } else if command == "hash" {
        // the second argument is the String to hash
        let password = &arg[2];

        hash(password);
    } else if command == "verify" {
        // the second argument is the String to hash
        let password = &arg[2];

        // the third argument is the hash to verify against
        let hash = &arg[3];

        verify(hash, password);
    } else {
        println!("Invalid command");
    }
}

// Creates a password out of random characters. The length is given as an argument.
fn random(len: usize) {
    // create a random number generator, from the rand crate.
    let mut rng = rand::rng();

    // This is a bit tricky. It uses a range, from 0 to len (which are just the numbers), then maps
    // each number into a random character, then collects the result back into a String.
    let chars: String = (0..len).map(|_| rng.sample(Alphanumeric) as char).collect();

    // Print the result.
    println!("Random chars: {}", chars);
}

// Create a passphrase from a wordlist, with len passed it as an argument to determine how many
// words our phrase has
fn phrase(len: usize) {
    // create a random number generator first
    let mut rng = rand::rng();

    // read in a wordlist file. You can look at it, it's just a huge list of words seperated by new
    // lines.
    let words = std::fs::read_to_string("eff_large_wordlist.txt").unwrap();

    // Again, a tricky iterator: we split the whole big string of all the words into lines, then
    // use choose_multiple from the rand crate to pick `len` words out of that list randomly. The
    // result is a Vec of strings.
    let phrase = words.lines().choose_multiple(&mut rng, len);

    // Join takes a vec and puts it all together with the given separator, in this case a hyphen.
    // That's our phrase!
    let phrase = phrase.join("-");

    // Print the phrase.
    println!("Random phrase: {phrase}");
}

// Generates an Argon2id hash from a password string
fn hash(password: &str) {
    // create a random salt. This isn't really secret.
    let salt = SaltString::generate(&mut OsRng);

    // create the default Argon2 object and use it to generate the hash from the password and salt
    let argon2 = Argon2::default();

    // or make our own, and set some options to make this more expensive to run
    // let a = Algorithm::Argon2id;
    // let v = Version::V0x13;
    // let p = ParamsBuilder::new()
    //     .m_cost(19_456u32)
    //     .t_cost(128)
    //     .p_cost(1)
    //     .build()
    //     .unwrap();
    // let argon2 = Argon2::new(a, v, p);

    // hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // print the result
    println!("{}", password_hash);
}

// verify a password against a given hash
fn verify(password_hash: &str, password: &str) {
    // Convert the hash into something Argon2 can use internally
    let parsed_hash = PasswordHash::new(password_hash).unwrap();

    // verify that hash against the given password
    let is_good = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    match is_good {
        true => println!("Password is correct!"),
        false => println!("Password is incorrect!"),
    }
}
