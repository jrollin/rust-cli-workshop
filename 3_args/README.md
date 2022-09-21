# Learn how to parse arguments

## :dart: Objectives


* add dependencies
* parse arguments 
* errors handling
* use modules
* type conversion


## Retrieve cli arguments

Rust already imports common used symbols like Vec, String, Option and Result.\
For other symbols, you must import module with `use` keyword

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

```bash
$ cargo run

     Running `/your/path/target/debug/crabby`
["/your/path/target/debug/crabby"]

$ cargo run -- foo bar 1

     Running `/your/path/target/debug/crabby`
["/your/path/target/debug/crabby", "foo", "bar", 1]
```

:bulb: Notes

* every arguments after `--` are passed to the program. \
Avoid mixing argument with `cargo run` program options (ex: cargo run --help)\
Equivalent to `./target/debug/crabby foo bar 1`
* [documentation about prelude](https://doc.rust-lang.org/std/prelude/index.html)


Common libraries provided in Rust standard library 
* fmt
* env
* io 
* path
* fs

[more about std library](https://doc.rust-lang.org/std/)


## Manage Errors

Update our code to greets with argument passed to our app

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    greets(&args[1])
}

fn greets(name: &str) {
    println!("Hello, {} 🦀 !", name);
}

```


```bash
$ cargo run -- John

Running `/your/path/target/debug/crabby_args john`
Hello, john 🦀 !
```

What happens if no argument is provided ?

<details>
<summary>Check by yourself to see error</summary>

```bash
$ cargo run -- 

thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1'
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
</details>


### We need to manage errors gracefully

Vectors implements `get` api to retrieve element by position ([documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get))


```rust
// from api doc
let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));
```

Either we have an argument provided either none

Optional can have a value `Some(val)` or `None`

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Many ways to manage our code 


```rust
// manage both cases
match args.get(1) {
    Some(name) => greets(name),
    None => panic!("No name provided!"), //quit program
}

// if / let  
if let Some(name) = args.get(1) {
    greets(name);
} else {
    eprintln!("No name provided");
}

// unwrap
// `unwrap` returns a `panic` when it receives a `None`.
let name = args.get(1).unwrap();

// expect
// panic like unwrap but with message
let name = args.get(1).expect("Name is required");
```

:bulb: More about flow of control : 
* [Optional doc](https://doc.rust-lang.org/rust-by-example/std/option.html?highlight=option#option)
* [if/let flow](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
* [panic or not panic](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)


## Retrieve application argument  

Try to pass an integer as argument and print square 

What happens when we execute this code ?

```rust
let x: u32 = args.get(1).unwrap();
println!("{} square is {}", x , x*x );
```

<details>
<summary>Try before reveal</summary>

```bash
let x: u32 = args.get(1).unwrap();
  |                ---   ^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `&String`
  |                |
  |                expected due to this
```

Arguments passed to app are Strings, you need to cast String to integer

</details>

### Parse application argument 

Try to `parse` String into integer can fail\
We can either have a Succes either an error

String parse method returns `Result`:

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

Examples of usage 

```rust
// Ok 
let four: u32 = "4".parse().unwrap(); // explicit type annotation
let four = "4".parse::<u32>().unwrap(); // turbofish annotation

//Err 
let notaninteger = "whatever".parse::<u32>().unwrap();
let toomuch = "300".parse::<u8>().unwrap();
```

We can manage error like we do with `Option`

Nb: `Err`  provides message about failure

```rust
match args.get(1).unwrap().parse::<u32>() {
    Ok(x) => println!("square of {} is {}", x, x * x),
    Err(e) => println!("error parsing : {}", e),
}
```

:bulb: Notes 
* [Result](https://doc.rust-lang.org/std/result/)
* [Str parse method](https://doc.rust-lang.org/std/primitive.str.html#method.parse)


## Organize code in modules

Modules and use Let you control the organization, scope, and privacy of paths

Let split our code in differents files

```bash
|-src
  |- main.rs
  |- greetings.rs
```

```rust
// greetings.rs
pub fn greets(name: &str) {
    println!("Hello, {} 🦀 !", name);
}
```
Nb: functions are private by defaut, you must add `pub` keyword


```rust
// main.rs
use std::env;

mod greetings; //declare module 
use greetings::greets; // import function

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = args.get(1).expect("Name is required");
    greets(&name);
}
```



More about modules :

* [crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)



### Type conversion



Use From

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```


Use tryFrom when conversion can fail

It uses Result enum type

```rust 
// example from doc.rs
struct GreaterThanZero(i32);

impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}

// Returns an error because `big_number` is too big to
// fit in an `i32`.
let try_smaller_number = i32::try_from(big_number);
assert!(try_smaller_number.is_err());

// Returns `Ok(3)`.
let try_successful_smaller_number = i32::try_from(3);
assert!(try_successful_smaller_number.is_ok());
```

:bulb: notes 

* [TryFrom documentation](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
* [convert module documentation](https://doc.rust-lang.org/std/convert/index.html)

### :pencil: Exercice 1 : move your previous code to modules

Expected directory tree :

```bash
|-src
  |- main.rs
  |- greetings.rs
  |- chifoumi.rs
```

:bulb: Tips 

* do not forget visibility
* you need to declare module and import functions

### :pencil: Exercice 2 : Update application to execute two commands

```bash
cargo run greets You

Hello, You 🦀 !
```

```bash
cargo run chifoumi rock paper

p1 vs p2 : Lost
```

Read the compiler errors :) 

:bulb: tips
* String can be create from &str with `String::from()`
* &str can created from String with  `.as_str()̀`
* use type conversion to convert cli argument to Game `enum`


## :clap: Congrats, you understand some useful paradigm in Rust !

Check a solution with unit tests [here](./solution/src/main.rs) 

