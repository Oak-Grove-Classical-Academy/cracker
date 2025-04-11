use rand::Rng;
use rand::distr::Alphanumeric;
use rand::seq::IteratorRandom;
use std::env;

fn main() {
    // get all the command line arguments. The 0th element is the app name, 1st will be the
    // command, 2nd the length.
    let arg: Vec<String> = env::args().collect();

    // get the first argument, command
    let command = arg[1].clone();
    // get the second argument, length. We parse it into a usize type, which is just an unsigned
    // number.
    let password_length = arg[2].parse::<usize>().unwrap();

    // figure out which command the user chose, and run the respective function.
    if command == "random" {
        random(password_length);
    } else if command == "phrase" {
        phrase(password_length);
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
