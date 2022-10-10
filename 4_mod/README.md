# Learn how organize code

Our cli have two functionalities (greetings and chifoumi)\
It is always better to split logic in different files


## :dart: Objectives

* use modules


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

:pushpin: Remember

* declare module and declare usage
* visibibility is private by default

:books: More resources

* [crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)


## :pencil: Exercice: Split app in different modules


Expected tree 


```bash
|-src
  |- main.rs
  |- greetings.rs
  |- chifoumi.rs
```

:bulb: Tips

* do not forget visiblity

## :clap: Congrats

you understand how to organize code in modules

Check a solution with unit tests [here](./solution/src/main.rs) 

## :memo: Summary

What you have learned

* modules
* visibility

### Next Part 

[:call_me_hand: Go to next part: Clap](../5_cli/README.md)
