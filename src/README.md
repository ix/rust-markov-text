# markov-text
A crate for Markov text generation.

Example:

```rust
extern crate markov_text;

use markov_text::{Dictionary, Sentence};
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut markov = Dictionary::new();
  let mut file = File::open("example").expect("Couldn't open `example` file.");
  let mut buf = String::new();

  file.read_to_string(&mut buf).expect("Failed to read the file!");
  markov.parse(Sentence::from(&*buf)).unwrap();

  let prefix = markov.rand_prefix().expect("Failed to get a prefix!"); {
    println!("{}", markov.generate(&prefix, 60).unwrap());
  }
}
```

Or, once you've created a dictionary and parsed some text to it, you can generate from your own "prefix", that is, two words (though this may be configurable in the future) as so:

```rust
let prefix = ("the".into(), "game".into()); {
  println!("{}", markov.generate(&prefix, 60).unwrap());
}
```
