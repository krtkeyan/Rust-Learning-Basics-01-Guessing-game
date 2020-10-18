## Rust Learning ( Basics ) - Building a Guessing Game

So we are familiar with how to run our program. Let's start building our guessing game - I make a number between 1-100 and your task is to guess it. All I can give you is a hint, that the number you had guessed at the moment is less or greater than the secret. 

So let's do a quick overview, what all we need to build this game.
1. We need to generate a random secret number.
2. We need to accept our input
3. Compare our input with secret number.
4. Process goes on, until we do our math to finally found the secret number.

#### Things covered 
1. `use` statement
2. Storing values in variables
3. Get input from terminal
4. Handling failures
5. Printing values with placeholders / substitution

---

#### 1. `use` statement

Rust only imports few things that are common to all programs - they are known as "prelude". There are other preludes like `std::io::prelude` that needs not to `use`ed manually, unlike default `preludes `use`ed automatically.

Since we need to get input from user, we need to include that in the scope. Otherwise, rust can't find the function needed for execution.

So we are going to add `use` statement, to include `std::io` library, that will helps us to get input from standard input ( terminal ).

Let's add `use std::io` above the main function.

#### 2. Storing values in variables

Before we get input from terminal, we need to store the value from terminal into a variable, to use it in our program.

To create a string variable, we can use `let variable_name = String::new()`. `let` helps us to create a variable. `String::new` function returns the instance of `String` type.

One thing to remember is, rust by default creates immutable variables, meaning values cannot be changed. Our guess is a repeated process, that we need to overwrite previous value, which means we do some mutation here. To make it mutable we need to add `mut` after `let` part.

So, let's create our guess variable and make it as a string.

```rust
    let mut guess = String::new();
```

#### 3. Get input from terminal

First thing, let's get an idea how can we give input to the guessing game. To do that we need to know how to read input from terminal.

```rust
    io::stdin().read_line(var)
```

Since we already bought, `io` into scope by adding, `use` at the top, we can access the `stdin` function.

`::` operator is path separator, used to separate crate, modules and items. Here `std` is a crate and `io` is a module. Maybe think of it as a `/` in `lodash/sort`. `lodash` is a package and it hold the sort module.

If we didn't add the `use` at the top, we can rewrite it as,

```rust
    std::io::stdin().read_line(var)
```

`stdin` function returns an instance of `std::io::Stdin`, handle that represents the standard input from terminal. `read_line` is a function that is part of that handle, which reads the standard input from terminal and save it in `var`.


To save the input into our `guess` variable, we can write our program as,

```rust
    io::stdin()
        .read_line(&mut guess);
```

The value returned from terminal will be saved into the reference of guess. It's like `scanf("%d",&a)` in C, you save it in the reference / location of `guess` variable without additional memory.

`&mut` before guess, is to instruct that it will be mutable as references also immutable by default. Since we are going to read input from user, repeatedly into the `&guess`.

#### 4. Handling failures

Let's make our program resilent, handling potential failures.

`read_line` function returns the instance of `io::Result` of type `enum` - it means value can be either `Ok` - for success or `Err` - For failure. We are going to add `expect` function at the end which captures in case of failure and displays the message given.

Our program becomes,

```rust
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get number");
```

#### 5. Printing values with placeholders / substitution

Let's print out the value we entered in terminal and currently stored in guess. `{}` can be used within our print statement, to replace with actual value. It's like `printf("%d",a)` in C.

Our final program, so far in this chapter will be,

```rust
    use std::io;

    fn main() {
        println!("What's your guess (1-100)");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get number");

        println!("Input given is {}",guess);
    }
```

Time to check our progress so far, run `cargo run` to see the output.

Next, we will generate a random secret and compare with our guess.

Run, `git checkout 03-external-dependencies-and-match-expr` to go to next chapter or in gitpod.
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://github.com/krtkeyan/Rust-Learning-Basics-01-Guessing-game/tree/03-external-dependencies-and-match-expr)
