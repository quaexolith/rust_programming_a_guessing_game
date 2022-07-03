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
