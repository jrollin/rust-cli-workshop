use std::fmt::{self, Debug, Display, Formatter};

fn main() {
    // string
    chars_unicode();
    // iterators and ownership
    iterator_ownership();
    // derive vs impl
    derive_vs_impl();
}

fn chars_unicode() {
    println!("Rust unicode strings: ");
    // str
    let greetings = "Hello you !";
    println!("{greetings}: {}", greetings.len());
    // nb:
    // ASCII: 7 bit characters encoding (0-127)
    // UTF-8: every code point from 0-127 is stored in a single byte

    // with unicode
    let greetings = "Hello you ♪!";
    println!("{greetings} length: {}", greetings.len());
    println!(
        "{greetings} length: {:?}",
        greetings.bytes().collect::<Vec<_>>()
    );
    println!("{greetings} chars count: {}", greetings.chars().count());

    // manipulate unicode
    let note = "♪";
    let bytes = note.as_bytes();
    println!("♪  bytes : {:?}", bytes);
    let s = String::from_utf8_lossy(bytes);
    println!("♪ valid utf8: {}", s);

    // from valid utf8 vec
    let buf = &[226, 153, 165];
    let s = String::from_utf8_lossy(buf);
    println!("valid utf8: {}", s);

    // invalid utf8 input
    let input = b"Hello \xF0\x90\x80World";
    let output = String::from_utf8_lossy(input);
    println!("invalid utf8: {}", output);
}

fn iterator_ownership() {
    println!("Rust collection and borrowing:");

    // take ownership
    let collec_value = vec!["a", "b", "c"];
    // same as collec_value.into_iter()
    for x in collec_value {
        println!("val: {x}");
    }
    // moved so error...
    // println!("{:?}", collec_value);

    // borrow by reference
    let collec_ref = vec!["a", "b", "c"];
    // same as collec_ref.iter()
    for x in &collec_ref {
        println!("ref: {x}");
    }
    println!("ref is not consumed: {:?}", collec_ref);

    // mutable reference
    let mut collec_refmut = vec![1, 2, 3, 4];
    // same as collect_refmut.iter_mut()
    for x in &mut collec_refmut {
        *x *= 2;
    }
    println!("mutable reference : {:?}", collec_refmut);
}

fn derive_vs_impl() {
    println!("Rust Debug and Display implementation:");
    let w = Whatever::new("About derive");
    println!("Display : {}", w);
    println!("Debug: {:?}", w);
    println!("Debug raw: {:#?}", w);
}

// dumb struct for trait
struct Whatever {
    inner: String,
}

// struct constructor convention
impl Whatever {
    fn new(inner: &str) -> Self {
        Self {
            inner: inner.to_string(),
        }
    }
}

impl Display for Whatever {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(Whatever : {})", self.inner)
    }
}

impl Debug for Whatever {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Whatever")
            .field("inner", &self.inner)
            .finish()
    }
}
