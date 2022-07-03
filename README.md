# rust_programming_a_guessing_game
Programming a Guessing Game in Rust

# Running the program
`cargo run`

# Syntax

## Passing by reference
`&` indicates passing by reference

```
.read_line(&mut guess)
```

## Variable declaration
```
let mut guess = String::new();
```

Variables are immutable by default. Declaring a variable with `mut` allows it to
be mutable.

## expect()

```
io::stdin()
      .read_line(&mut guess) // & indicates passing by reference
      .expect("Failed to read line");
```

`read_line` returns a `Return` enum. Possible variables: `Ok` and `Error`. These
are referred to as variants.

If this instance of `Result` is an `Ok` value, expect will take the return value
that `Ok` is holding and return just that value to you so you can use it. In
this case, that value is the number of bytes in the user’s input.

## rand::thread_rng
A function that gives a particular random number generator that is local to the
current thread of execution and seeded by the operating system.

## 1..=100
Specifies a range that is inclusive on the upper and lower bounds.

## match
A match expression is made up of arms. An arm consists of a pattern to match
against, and the code that should be run if the value given to match fits that
arm’s pattern. Rust takes the value given to match and looks through each arm’s
pattern in turn.

Using an `_` underscore in a match arm is a catchall value.

## Integers
Integers are of type `i32`, a 32-bit integer, unless otherwise specified.

## Variable Shadowing
Variables can be shadowed to convert a variable from one type to another.

# Central Repo
https://crates.io/

This is akin to Maven central for all you Java nerds like me.

# Documentation
`cargo doc --open`
