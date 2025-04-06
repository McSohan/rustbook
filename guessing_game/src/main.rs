use rand::Rng;
// this is just a "trait" and it needs to be in focus for us to use the methods defined by rand
use std::io;
use std::cmp::Ordering;
// Bringing a type called std::cmp::Ordering into scope from the standard library.

fn main() {
    println!("Guess the number");
    //std::io::stdin() returns a handle to an instance of Stdin from which we can read

    let start_range = 1;
    let end_range = 100;

    let secret_number = rand::thread_rng().gen_range(start_range..=end_range);
    // use the random number generator which is local to the current thread. It is seeded by the operating system
    // start..=end is a range expression inclusive of start and end
    // println!("The secret number is {secret_number}");
    // we do not want to print the secret number anymore
    loop {
        println!("Guess the number now:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Trim eliminates all the initial and the terminal whitespaces.
        // Rust also infers that secret_number is now u32 -- this is because of the comparision made with guess
        // : u32 is annotating the type of the variable
        // We are changing from an expect call to a match expression
        // _ is a catchall value

        /*
        Rust allows us to shadow the previous value of guess with a new one.
        Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables,
        such as guess_str and guess.
        */

        println!("You guessed {guess}");
        // Rust is strongly and statically typed
        match guess.cmp(&secret_number) {
            // This is made up of arms - each of which is a pattern to match against.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/*
variables are immutable by default in rust and we have to explicitly make them mutable.
references are also immutable my default, and hence we need to write mut to make them mutable.
That cargo.lock file is important and needs to obe checked into the VCS,
    this is because specific version dependencies to make the code work are stired in the .lock file.

*/
