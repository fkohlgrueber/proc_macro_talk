# WIP!!!

# (Function-like Procedural Macros)

- Declarative `macro_rules!` macros
- Procedural macros
  - Attribute-like macros
  - Custom derive macros
  - Function-like macros

# Declarative macros

Definition:

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

Usage:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

# Procedural macros

Attribute-like:

```rust
#[route(GET, "/")]
fn index() {
    ...
}
```

Attribute-like macro:

```rust
#[route(GET, "/")]
fn index() {
    ...
}
```

Custom derive macro:

```rust
#[derive(HelloMacro)]
struct Pancakes;
```

Function-like macro:

```rust
my_macro!{
    <div>Hello World!</div>
}
```

