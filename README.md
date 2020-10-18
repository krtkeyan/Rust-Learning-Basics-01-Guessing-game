## Rust Learning ( Basics ) - Building a Guessing Game
In previous step, we learned to read input from the user and have it saved to `guess` variable. Now, we need to generate our secret number and have it compared to the guess.


#### Things covered 
1. Using external crate
2. Generating random secret using `rand` crate
3. Comparing using `match` expression
4. Shadowing & `parse` function to typecase string to number type.
5. Matching out our first guess !
---

### 1. Using external crate

Rust, doesn't support random number generation by default. But we can make use of external crates to extend the functionality to our program.

Add the following snippet to your `Cargo.toml` configuration file and install [rand](https://crates.io/crates/rand).
```
    [dependencies]
    rand = "0.7.2"
```
Now run `cargo build` to download crate into your package. 

### 2. Generating random secret using `rand` crate

Now, let's generate our secret number. Just like `stdin` function needs `std::io` in it's scope. `rand::thread_rng()`, generates random number local to current thread. 

This function followed by `gen_range` method, generates random number between two ranges, needs to define `rand::Rng` trait at its scope, as it requires this trait to implement in its scope ( Traits are like interfaces that need to be implemented ). 

So, 
  1. We add `rand::Rng` trait to scope, by `use rand::Rng` at top.
  2. `let secret = rand::thread_rng().get_range(1,100)`, in our main function to generate and save it to a variable.

### 3. Comparing using `match` expression

To compare, guess and secret, we can use `cmp` method - `guess.cmp(&secret)`. The call to `cmp` method returns `std::cmp::Ordering` . `Ordering` is an enum type, with values `Less, Greater or Equal`. We need to handle the three possible outcomes, to determine what to do next.

`match` expression can be used to do that. It consists of arm, with pattern first and second what to do if pattern matched.

So let's write some code now,

```rust
    match guess.cmp(&secret) {
        std::cmp::Ordering::Less => println!("You Guess, is less than secret."),
        std::cmp::Ordering::Greater => println!("You Guess, is greater than secret."),
        std::cmp::Ordering::Equal => println!("Bingo!. You made it"),
    }
```
Instead of repeating `std::cmp` to bring into scope for enum values `Less` we can add it to top as `use std::cmp::Ordering`;

Let's run our code, so far using `cargo run`.

#### 4. Shadowing & `parse` function to typecase string to number type.

You might find our program, throws error regarding type mismatch. `cmp` method expects both types to be same.
Let us change our `guess` variable to number type, to match it with secret type. `guess.trim().parse()` method first trims new-lines from standard output, `parse` method on strings changes it to number.

What if, we gave string as input ? We might want to handle that too. Add `expect("Only number is accepted")` to the parse, that return `Result` type.

Now, we need to save this value to a variable. Our original `guess` variable is of `String` type, we can shadow the original variable with new number type ( u32 - unsigned 32bit integers as only positive numbers are in our range ).

We can shadow it by,

```rust
let guess: u32 = guess.trim().parse().expect("Only number is accepted");
```

#### 5. Matching out our first guess !

Let's run our program, with `cargo run` to check out our first guess.

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1,100);

    println!("What's your guess (1-100)");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get number");

    let guess: u32 = guess.trim().parse().expect("Only number is accepted");
    println!("Input given is {}",guess);

    match guess.cmp(&secret) {
        Ordering::Less => println!("You Guess, is less than secret."),
        Ordering::Greater => println!("You Guess, is greater than secret."),
        Ordering::Equal => println!("Bingo!. You made it"),
    }
}
```

That's great ! We made our program to check our guess with the secret number. 

But, it seems we might only do one guess, we need our program to run until we guess out. Next, we will finish up our guessing game, by making our program to keep asking our input until it matches out. 


Run, `git checkout 04-loops-and-finish-up-guessing-game` to go to next chapter.
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://github.com/krtkeyan/Rust-Learning-Basics-01-Guessing-game/tree/04-loops-and-finish-up-guessing-game)
