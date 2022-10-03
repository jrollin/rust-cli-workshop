# Learn Rust basic syntax

## :dart: Objective 

Manipulate Rust syntax needed for our CLI

* variable declaration and assignation
* basic types
* compound types
* operator 
* statement vs expression 
* unit test

## :pencil: Part 1 - Understand main Rust syntax / types 

Let discover some Rust Syntax and concepts

:bulb: Notes 

Take time to read this part\
You can refer to it when writing exercice implementation\
Everything needed is described below.

### Variables and assignations 

```rust 
let x; // declare "x"
x = 10; // assign 10 to "x"


// declare with assign
let x = 10; // The default integer type in Rust is i32
let y: i8 = -128;

let x = 1.5; // the default float type in Rust is f64
let y: f64 = 2.0;

//cast
let x: i32 = -1200
let y: u16 = 6547
let res: f64 = x as f64 / y as f64     


// two equivalent syntaxes
let a = 5i8; 
let a : i8 = 5;

// two equivalent syntaxes
let b = 100_000_000;
let b = 100000000;

// casting
let foo = 3_i64;
let bar = foo as i32;

// no changing value 
const FOREVER_AGE: u8 = 18;
static LANGUAGE: &str = "Rust";

// mutability
let age = 5; // declare variable
age = 18 //boom, variables are immutable by default

let mut age = 6; // add mut keyword
age = 5; // ok 
```

```rust
print!("display line without newline")
println!("display with placeholders {} and new line", 1);

// pattern options
{}: std::fmt::Display trait
{:?} : std::fmt::Debug trait
{:p}  : format the variable as a pointer and prints the memory address that the value points to.
{:032b} means to format as a binary via the std::fmt::Binary trait with 32 zeroes padded on the left

```

```rust
// syntax to specifiy numbers 
123_456   // underscore as separator
0x12      // prefix 0x to indicate hex value
0o23      // prefix 0o to indicate octal value
0b0001    // prefix 0b to indicate binary value
b'a'      // A single byte character

// example with formatting print
println!("Base 10 repr:               {}",   69420);
println!("Base 2 (binary) repr:       {:b}", 69420);
println!("Base 8 (octal) repr:        {:o}", 69420);
println!("Base 16 (hexadecimal) repr: {:x}", 69420);
println!("Base 16 (hexadecimal) repr: {:X}", 69420);

```

:pushpin: Remember

* variables are immutable if no `m̀ut` keyword specified
* type can be explicit or infered when possible for compiler
* println! can format types to text 

:books: More resources

* [variables and mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
* [formatting options](https://doc.rust-lang.org/stable/std/fmt/index.html#formatting-traits)



## Strings vs slices 


```rust 
// String literals, not mutable
let x: &str = "hello world!";  // note lowercase syntax "str"

// String 
let s: String = "Hello world".to_string(); // camelCase syntax
// another way to build String
let s: String = String::from("Hello world"); //  


// concatenation needs mutable String
let mut hello = String::from("Hello, ");
hello.push('w');
hello.push_str("orld!");

// string block
let json = r#"
        {
          "name": "George",
          "age": 27,
          "verified": false
        }
"#;

``` 

:pushpin: Remember

* Strings literals are not mutable (stored on stack)
* Strings is more conveniant than &str but less "performant" (stored on heap)

:books: More resources

* [Slices vs Strings](https://doc.rust-lang.org/book/ch04-03-slices.html)
* [references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Vectors, arrays, tuples

```rust
// A fixed-size array of four i32 elements
let four_ints: [i32; 4] = [1, 2, 3, 4];

// A dynamic array (vector)
let mut vector: Vec<i32> = vec![1, 2, 3, 4]; // vec! is a macro
vector.push(5);

//tuples
let x: (i32, &str, f64) = (1, "hello", 3.4);

// Destructuring `let`
let (a, b, c) = x;
println!("{} {} {}", a, b, c); // 1 hello 3.4
``` 

:pushpin: Remember

* array have fixed size 
* vector have dynamic size
* destructing is commonly used in match  
* functions with `!` like `vec![]` are macro. Rust replace it with code at compilation

:books: More resources 

* [destructuring](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html)
* [arrays and slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html) 
 
## Types 

```rust

// Struct
struct Point {
        x: i32,
        y: i32,
}
let origin: Point = Point { x: 0, y: 0 };

// A struct with unnamed fields, called a ‘tuple struct’
struct Point2(i32, i32);
let origin2 = Point2(0, 0);

// enum
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
let left = Direction::Left;

// enum with values
enum Movement {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

// enum with struct values
enum Actions {
    StickAround,
    MoveTo { x: i32, y: i32},
}
```

:pushpin: Remember

* Rust do not mix data and behaviour. You don't have "classes" like in Java
* `enum` are powerful and commonly used with `match`  operator

:books: More resources

* [structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
* [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)

## Optional and Result 

Rust have to specific enum already defined

```rust
// An output can have either Some value or no value/ None.
enum Option<T> { // T is a generic and it can contain any type of value.
    Some(T),
    None,
}

// A result can represent either success/ Ok or failure/ Err.
// T and E are generics. T can contain any type of value, E can be any error.
enum Result<T, E> {     
    Ok(T),
    Err(E),
}

let v = vec![10, 20, 30]; // initialization macro    
let idx = 0;
match v.get(idx) {
        Some(value) => println!("Value is {}", value),
        None => println!("No value..."),
}
```

:pushpin: Remember

* There are no Exception in Rust. Either you have a succesful operation  or an Error
* There are non Null or Void in Rust.  Either you have a value or an absence of value  

:books: More resources 

* [Option](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html)
* [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
* [error handling](https://doc.rust-lang.org/rust-by-example/error.html)

## Functions and iterator 

```rust
fn is_even(i: i32) -> bool {
    i % 2 == 0
}

// fizzbuzz
if x % 3 == 0 && x % 5 == 0 {
    println!("FizzBuzz")
} else if x % 3 == 0 {
    println!("Fizz")
} else if x % 5 == 0 {
    println!("Buzz")
} else {
    println!("{}", x)
}

// pattern matching 
 match (self % 3, self % 5) {
    (0, 0) => String::from("FizzBuzz"),
    (0, _) => String::from("Fizz"),
    (_, 0) => String::from("Buzz"),
    _ => format!("{}", self),
} 


fn main() {
    let sum: i32 =
        (0..5)                   // this is an iterator
        .filter(|i| is_even(*i)) // filter with a closure
        .sum();                  // consume the iterator
        
    println!("sum of even numbers is {}", sum);
}
```

:warning: There are no null value in Rust

```rust
// The empty tuple () represents the absence of data.

fn whatever() -> () {}
```


:pushpin: Remember

* statements need `;` keyword
* note the absence of `return whatever ;` keyword when evaluating expression 
* match operator evaluates all possible values

:books: More resources

* [expression](https://doc.rust-lang.org/rust-by-example/expression.html)
* [functions](https://doc.rust-lang.org/rust-by-example/fn.html)
* [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

## :pencil: Exercice 1 - Write a simple function with Unit Test


```rust 
// main.rs
// code stripped


fn add(a: u8, b: u8 ) -> u8 {
   a + b
}

```

add tests module in your main file

```rust 
// main.rs
// code stripped

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(12, 5), 17);
    }
}

```

Run test with cargo 

```Bash
cargo test
```

:bulb: Notes 
 
* `#[..]` annotation is a `derive attribute` (another macro like `println!`) 
* `mod` define a new modules
* `cfg(test)` indicates that this part of code is only used in testing context


:tada: Congrats, you have written your first Unit test ! 



### Let's play with our code 

Let make some changes in our previous function


#### 1) Add test with values higher than 255  ?

Change signatures and launch test

<details>
<summary>&#128073; Check <code>cargo test</code>  output </summary>

```bash
   |
   |         assert_eq!(add(300, 5), 17);
   |                        ^^^
   |
   = note: `#[deny(overflowing_literals)]` on by default
   = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
```
</details>


#### 2) Replace u8 with i64


Change first argument signature with `i64` and launch test


<details>
<summary>&#128073; Check <code>cargo test</code> output </summary>


```bash
error[E0308]: mismatched types
 --> 2_syntax/solution/src/main.rs:6:9
  |
6 |     a + b
  |         ^ expected `i64`, found `u8`

error[E0308]: mismatched types
 --> 2_syntax/solution/src/main.rs:6:5
  |
5 | fn add(a: i64, b: u8) -> u8 {
  |                          -- expected `u8` because of return type
6 |     a + b
  |     ^^^^^ expected `u8`, found `i64`
  |
help: you can convert an `i64` to a `u8` and panic if the converted value doesn't fit
  |
6 |     (a + b).try_into().unwrap()
  |     +     +++++++++++++++++++++

error[E0277]: cannot add `u8` to `i64`
 --> 2_syntax/solution/src/main.rs:6:7
  |
6 |     a + b
  |       ^ no implementation for `i64 + u8`
  |
  = help: the trait `Add<u8>` is not implemented for `i64`
  = help: the following other types implement trait `Add<Rhs>`:
            <&'a f32 as Add<f32>>
            <&'a f64 as Add<f64>>
            <&'a i128 as Add<i128>>
            <&'a i16 as Add<i16>>
            <&'a i32 as Add<i32>>
            <&'a i64 as Add<i64>>
            <&'a i8 as Add<i8>>
            <&'a isize as Add<isize>>
          and 48 others

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `crabby_syntax` due to 3 previous errors
```
</details>


#### Embrace changes with compiler love

Rust compiler warning and hints help you detect and fix problems very fast

:bulb: Notes 

* Compiler is really an helping tools that explicits reason of failure 
* Most of the time, it will give a solution or hint and a link to related documentation
* some consider rust compiler as your pair 

## :pencil: Part 2 - Write a chifoumi game logic 


Write Chifoumi game logic (Rock, Paper, Scissors)  

* Rock beats scissors
* Scissors beats paper
* Paper beats rock


Write the missing code

```rust
#[derive(Debug)]
enum Game {
    //your code
}

#[derive(Debug, PartialEq)]
enum GameResult {
       Win,
       Lost,
       Draw
}

fn play( a: ?, b: ?) -> GameResult {
    // your code
} 


fn main() {
    // define your games a and b
    // call play function with arguments
    // display result 
}
```

:bulb: Notes 

* `#[derive(Debug)]` :  asks the compiler to auto-generate a suitable implementation of the Debug trait to display Struct
* `#[derive(PartialEq)]` : asks the compiler to enables use of the == and != operator by implementin 'PartialEq' trait (Useful in trait) 

Display variable without custom implementation, you can add `#derive(Debug)̀`

```rust
println!("Debug {:?}", variable);
```

<details>
<summary>&#128073; Implementation Tips </summary>
Instead of using if/else to compare values, you can use match operator 


```rust
// make a tuple with both values
match (a, b) {
    (Game::Rock, Game::?) => ... code ... 
}
```

using enum, you cannot miss matching cases without compiler warning

```rust
//compiler error example
error[E0004]: non-exhaustive patterns: `(?, ?)` not covered
```
</details>



## :clap: Congrats

You understand enough syntax and Rust Concepts to code a chifoumi

Check a solution with unit tests [here](./solution/src/main.rs) 

## :memo: Summary 

What you have learned

* declare and assign variables
* express how to store data with primitives or custom types  
* define functions
* write and launch unit test


