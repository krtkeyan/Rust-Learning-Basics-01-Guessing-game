## Rust Learning ( Basics ) - Building a Guessing Game

In this branch, we are going to learn some basics, before we go to build our guessing game. If you already set-up the installation and know how to build and run your rust project. You may go for next chapter.

#### Things covered 
1. Install Rust in your system
2. An overview about Cargo 
3. Hello, Superstar 😎 !
4. Compile your first project
5. Run your first project
6. Cargo, Run!

#### 1. Installation

In case, if you have not done the installation part, short steps to do that.

Run, `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`. Once the installation succeed, `Rust is installed now. Great!` message will show up.

#### 2. Cargo - Package manager

Cargo, is rust's build-in package manager. Like npm for node.js. You can check, cargo installed in your system. By running, `cargo --version`.

You can initialise a project by running, `cargo new <package-name>`. `cargo new hello_cargo` would create you a folder named `hello-cargo`, with folder structure setup.

`Cargo.toml` is the configuration file for the project, just like `package.json` for node.js.

> This branch has been already configured with the basic structure, so you don't want to run the above steps.

#### 3. Hello, Superstar 😎 !

Let's get into coding part and let's keep it simple. We are just gonna print your name.

Cargo, looks into your `src` folder and start from `main.rs`. We are going to edit that file, now!

*Rust, starts execution from `main` function.*

So, we are going to create our main function first. `fn` defines the function, followed by function name - `main` in our case.

`fn main() {
    
}`

Great, we got our function. Let's make it to print your name next.

`println!("Hello, Superstar 😎 !"); // You are really a Superstar`

*println is a macro. Because, it ends with an !(exclamation) - Just remember that. It's not a function as it seems`

We use `println!("")` to print whatever we want. Here we make it to print your name. 

Go to the `src/main.rs` and follow the above steps.

#### 4. Compile your first program

So, next we want to see it in action, right ! To do so, we are going to look some commands. Rust, let's you build a binary that can be shared with anyone, they don't need to have rust installed in their system, unlike .py or .rb

To compile a rust file, run `rustc src/main.rs`. It will generate a binary file `main` in current directory.

#### 5. Run your first project

The binary file can be executed by `./main`. You can see the output in your terminal.

#### 6. Cargo, Run!

Cargo, simplifies the way you build your rust application.

You can compile and build and run your application in one step - `cargo run`. It compiles your application and let you know any errors, if any. And build your application and produce the output in `target/debug`. You can find a binary file with package name as in `Cargo.toml`.

To build a production ready version, you can run `cargo run --release`

If you choose run manually, after compile & build, you can run `cargo build`.
If you choose to only compile and not generate any binary, you can run `cargo check`. It's useful if we want to check periodically, programs run without errors and it's fast.


So you have learned with starting your first program, let's jump into building our guessing game.

* Run `git checkout 02-getting-input-and-variables` to go to next step.
