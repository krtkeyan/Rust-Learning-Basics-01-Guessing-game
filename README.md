## Rust Learning ( Basics ) - Building a Guessing Game
We have made our program to validate our first guess. But to make it more fun, we need to make our program to allow us more guessing until we manage to guess out the secret. Let's add this one final part, and try out our finishing game.

### Things covered
1. Loops and break 
2. Error handling - Handling invalid inputs
3. The Guessing game !

---

### 1. Loops and break 

We can make our program, to run repeatedly by wraping the statements with `loop`.

```rust
    loop {
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

Now, we can run it up! and see we have made our guessing game. We can play it, by guessing and narrowing down our secret with Less & Greater cue.

One thing, you might notice that, the program keep on running, even we hit our *Bingo*

To stop the program in that case, we might need to add `break` statement, which exits out of `loop`.

```rust
    match guess.cmp(&secret) {
        Ordering::Less => println!("You Guess, is less than secret."),
        Ordering::Greater => println!("You Guess, is greater than secret."),
        Ordering::Equal => {
            println!("Bingo!. You made it");
            break;
        },
    }
```

### 2. Error handling - Handling invalid inputs

Next thing, we might notice our program fails if we enter text instead of number. We should handle that case too.

`parse` method returns Result type of enum, Ok or Err. We might need handle to skip the current input in the loop and go to next iteration. `continue` statement tells program to go to next iteration of loop, skipping anything below the current execution.

First, we might want to `match` the Return types and act based on that. To do so,

```rust
    match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
    }
```

1. `Ok(num)` in the first arm pattern, will make match return the `num` value returned from `parse` method. 
2. `Err(_)` in the second arm, we can add `continue` to skip current iteration, `_` catches all `Err` values.


### 3. The Guessing game !

That's it. We can play with the guessing game by narrowing down our guess with help of cue. More fun, if we extend our secret range generated.

Final program will be,

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1,100);
        
    loop {
        println!("What's your guess (1-100)");
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only number is accepted :(, Try again!");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("You Guess, is less than secret."),
            Ordering::Greater => println!("You Guess, is greater than secret."),
            Ordering::Equal => {
                println!("Bingo!. You made it");
                break;
            },
        }
    }

}
```

---

We have learned the hands-on way to understand the basics of Rust. Let's have a recap of topic learned so far,

1. First steps, Basics of compiling & running with Cargo.
2. `use` statement - to bring methods, traits into scope
3. Creating variables, Shadowing variables ( Mutatable & Immutable )
4. Getting standard input from terminal
5. Printing value to the standard output
6. Adding external crates into the application
7. `match` expression and arm pattern for decision making
8. `loop` keyword, `break` and `continue`
9. Handling errors using `expect`

While we understand the origins of these concepts, next we can explore deep into these concepts to learn Rust.
Bon Voyage !
