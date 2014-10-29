# macros

A collection of rust macros.

### Conditionals

The `maybe!` macro returns a random boolean when called with empty parameters:

```rust
let rand: bool = maybe!();
```

When called with a block/expression, the block/expression is called if a random boolean is true:

```rust
maybe! {
    println!("50%-50% chance")
}
```

### Repetition

The `repeat!` macro is called with the format: `repeat! { <block or expression> <expression> times }`.
Multiple `<block or expression> <expression> times` groupings can be placed together in a `repeat!` macro, when seperated by a comma.

```rust
repeat! {
   println!("Double, double, toil and trouble") 3 times
}
```

The `twice!` macro calls the supplied block/expression twice:

```rust
twice! {
    println!("Happy Birthday to You")
}
```

### Hash literal

The `hash!` macro creates an `std::collections::HashMap` from `key => value` pairs:

```rust
let hash = hash! {"January" => 0u, "February" => 1, "December" => 11};
```

### Ternary operator

Ternary operators have been removed from rust, but the `t!` macro can be used as a substitute using this syntax:

```rust
t! { conditional ? true_branch : false_branch }
```

For example:

```rust
let val = t! { maybe!() ? "yay" : "nay" }
```
